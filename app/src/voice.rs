//! Voice pipeline: capture -> STT -> parser -> actions -> TTS.
//! Made by KebiLab

use anyhow::Result;
use kebi_core::{Config, Profile};
use kebi_llm::{LlmClient, providers::LlmProvider};
use kebi_stt::{SttEngine, WhisperApi, wake_word};
use kebi_ui::{VoiceController, VoiceEvent, VoiceState};
use std::sync::Arc;
use tokio::sync::mpsc;

pub fn spawn_pipeline(
    config: Config,
    _profile: Profile,
    controller: Arc<VoiceController>,
    tx: mpsc::UnboundedSender<VoiceEvent>,
) {
    tokio::spawn(async move {
        let buffer = Arc::new(parking_lot::Mutex::new(Vec::<i16>::new()));
        let buf_for_thread = buffer.clone();
        let wake_detected = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let (cap_tx, mut _cap_rx) = mpsc::unbounded_channel::<()>();

        let wake_word_enabled = config.general.wake_word_enabled;
        if wake_word_enabled {
            let _ = kebi_stt::WakeWordDetector::try_new("", &[]);
        }

        let cap_tx_for_thread = cap_tx.clone();

        // Capture thread – always runs, feeds buffer + sets wake flag
        std::thread::spawn(move || {
            let mut cap: Option<kebi_audio::AudioCapture> = None;
            loop {
                if cap_tx_for_thread.is_closed() {
                    break;
                }
                if cap.is_none() {
                    let mut c = kebi_audio::AudioCapture::new(16000);
                    let buf2 = buf_for_thread.clone();
                    if c.start(move |s| {
                        buf2.lock().extend_from_slice(s);
                    })
                    .is_err()
                    {
                        tracing::error!("audio start failed");
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                        continue;
                    }
                    cap = Some(c);
                }
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
        });

        let lang = config.general.language.clone();
        let max_phrase_samples = (config.audio.max_phrase_ms as usize) * 16; // 16 kHz
        let wake_word_phrase = config.general.wake_word.clone();

        // Main pipeline loop
        loop {
            // ── TRIGGER PHASE ──
            if wake_word_enabled {
                controller.state().set(VoiceState::WakeListening);
                let _ = tx.send(VoiceEvent::State(VoiceState::WakeListening));
                // Poll for either: wake word via Whisper probe, or manual button press
                let mut wake_tick: u64 = 0;
                loop {
                    // Cap buffer to ~2 s to prevent unbounded growth
                    {
                        let mut b = buffer.lock();
                        if b.len() > 32000 {
                            let excess = b.len() - 32000;
                            b.drain(0..excess);
                        }
                    }
                    let state = controller.state().get();
                    if state == VoiceState::Listening {
                        // Manual button press – skip wake detection
                        break;
                    }
                    // Every 1.2 s probe Whisper with what we have so far
                    if wake_tick % 6 == 0 {
                        let snapshot: Vec<i16> = buffer.lock().clone();
                        if snapshot.len() >= 8000 {
                            // Only probe if there's likely speech (>= 0.5s of audio)
                            // Skip probe if no STT key configured
                            if config.get_stt_api_key().is_some() {
                                let probe_text = run_stt(
                                    &config,
                                    snapshot.clone(),
                                    &lang,
                                )
                                .await
                                .unwrap_or_default();
                                if wake_word::contains_wake(&probe_text, &wake_word_phrase) {
                                    tracing::info!("wake word detected: {probe_text}");
                                    wake_detected.store(true, std::sync::atomic::Ordering::SeqCst);
                                }
                            }
                        }
                    }
                    if wake_detected.swap(false, std::sync::atomic::Ordering::SeqCst) {
                        controller.state().set(VoiceState::Listening);
                        let _ = tx.send(VoiceEvent::State(VoiceState::Listening));
                        break;
                    }
                    wake_tick = wake_tick.wrapping_add(1);
                    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                }
            } else {
                while controller.state().get() != VoiceState::Listening {
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
            }

            // ── CAPTURE PHASE ──
            buffer.lock().clear();
            while controller.state().get() == VoiceState::Listening {
                tokio::time::sleep(std::time::Duration::from_millis(80)).await;
            }
            tokio::time::sleep(std::time::Duration::from_millis(150)).await;

            let samples = {
                let mut b = buffer.lock();
                std::mem::take(&mut *b)
            };
            if samples.is_empty() {
                let next = if wake_word_enabled {
                    VoiceState::WakeListening
                } else {
                    VoiceState::Idle
                };
                let _ = tx.send(VoiceEvent::State(next));
                continue;
            }
            // Truncate very long captures
            let samples = if samples.len() > max_phrase_samples {
                samples[samples.len() - max_phrase_samples..].to_vec()
            } else {
                samples
            };

            // ── PROCESS PHASE ──
            let _ = tx.send(VoiceEvent::State(VoiceState::Recognizing));
            let text = match run_stt(&config, samples, &lang).await {
                Ok(t) => t,
                Err(e) => {
                    let _ = tx.send(VoiceEvent::Error(format!("STT: {e}")));
                    let _ = tx.send(VoiceEvent::State(VoiceState::Error));
                    tokio::time::sleep(std::time::Duration::from_millis(800)).await;
                    continue;
                }
            };
            if text.is_empty() {
                let next = if wake_word_enabled {
                    VoiceState::WakeListening
                } else {
                    VoiceState::Idle
                };
                let _ = tx.send(VoiceEvent::State(next));
                continue;
            }
            let _ = tx.send(VoiceEvent::Heard(text.clone()));
            let _ = tx.send(VoiceEvent::State(VoiceState::Thinking));
            let reply = run_command(&config, &text).await;
            let _ = tx.send(VoiceEvent::Reply(reply.clone()));
            if config.general.tts_enabled && !reply.is_empty() {
                let _ = tx.send(VoiceEvent::State(VoiceState::Speaking));
                let _ = speak(&reply, &config.tts.voice, config.tts.rate, config.tts.volume);
            }

            let next = if wake_word_enabled {
                VoiceState::WakeListening
            } else {
                VoiceState::Idle
            };
            let _ = tx.send(VoiceEvent::State(next));
        }
    });
}

async fn run_stt(config: &Config, samples: Vec<i16>, lang: &str) -> Result<String> {
    let api_key = config
        .get_stt_api_key()
        .ok_or_else(|| anyhow::anyhow!("STT API key is not set"))?;
    let mut stt = WhisperApi::new(
        config.stt.whisper_endpoint.clone(),
        api_key,
        config.stt.whisper_model.clone(),
        lang.to_string(),
    );
    stt.feed(&samples).await?;
    let text = stt.finalize().await?;
    Ok(text)
}

async fn run_command(config: &Config, raw_text: &str) -> String {
    use kebi_core::command::Command;
    use kebi_core::parser::ParserContext;

    let lang = if config.general.language == "en" {
        "en"
    } else {
        "ru"
    };
    let ctx = ParserContext {
        profile: &Profile::default(),
        wake_word: &config.general.wake_word,
        language: lang,
    };

    let text = strip_wake(raw_text, &config.general.wake_word);
    if text.is_empty() {
        return String::new();
    }

    let cmd = match kebi_core::parser::local::try_match(&ctx, text) {
        Some(p) => p.command,
        None => match llm_pick(config, text).await {
            Some(c) => c,
            None => Command::Unknown {
                reason: "no_match".into(),
            },
        },
    };

    match kebi_core::actions::execute(&cmd).await {
        Ok(Some(r)) => r,
        Ok(None) => String::new(),
        Err(e) => format!("Action error: {e}"),
    }
}

async fn llm_pick(config: &Config, text: &str) -> Option<kebi_core::command::Command> {
    use kebi_core::command::Command;
    let api_key = config.get_api_key()?;
    let provider = LlmProvider::from_code(&config.llm.provider);
    let base = if provider == LlmProvider::Custom {
        config.llm.base_url.clone()
    } else {
        provider.default_base_url().to_string()
    };
    let model = if config.llm.model.is_empty() {
        provider.default_model().to_string()
    } else {
        config.llm.model.clone()
    };
    let client = LlmClient::new(base, api_key, model, config.llm.timeout_secs);
    let messages = vec![
        kebi_llm::ChatMessage {
            role: "system".into(),
            content: kebi_llm::prompt::parse_system_prompt(),
        },
        kebi_llm::ChatMessage {
            role: "user".into(),
            content: format!("Command: {text}"),
        },
    ];
    let json = client.chat_json(messages).await.ok()?;
    let v: serde_json::Value = serde_json::from_str(&json).ok()?;
    serde_json::from_value::<Command>(v).ok()
}

fn strip_wake<'a>(input: &'a str, wake: &str) -> &'a str {
    let lower = input.to_lowercase();
    let w = wake.to_lowercase();
    if let Some(rest) = lower.strip_prefix(&w) {
        let cut = (input.len() - rest.len()).min(input.len());
        input[cut..].trim_start_matches(|c: char| c == ',' || c == ' ' || c == ':')
    } else {
        input.trim()
    }
}

fn speak(text: &str, voice: &str, rate: i32, volume: u8) -> Result<()> {
    use std::process::Command;
    if text.trim().is_empty() {
        return Ok(());
    }
    let escaped = text.replace('\'', "''");
    let script = format!(
        "Add-Type -AssemblyName System.Speech; \
         $s = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
         $s.SelectVoice('{voice}') | Out-Null; \
         $s.Rate = {rate}; $s.Volume = {vol}; \
         $s.Speak('{escaped}')",
        voice = voice,
        rate = rate,
        vol = volume,
        escaped = escaped,
    );
    Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .output()?;
    Ok(())
}
