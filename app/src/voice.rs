//! Voice pipeline: capture -> STT -> parser -> actions -> TTS.
//! Made by KebiLab

use anyhow::Result;
use kebi_core::{Config, Profile};
use kebi_llm::{LlmClient, providers::LlmProvider};
use kebi_stt::{SttEngine, WhisperApi};
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
        // Run microphone capture in a dedicated OS thread (cpal::Stream is !Send).
        let buffer = Arc::new(parking_lot::Mutex::new(Vec::<i16>::new()));
        let buf_for_thread = buffer.clone();
        let (cap_tx, mut cap_rx) = mpsc::unbounded_channel::<()>();
        let stop_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let stop_for_thread = stop_flag.clone();
        let cap_tx_for_thread = cap_tx.clone();
        std::thread::spawn(move || {
            // Initialize capture lazily on first listen.
            let mut cap: Option<kebi_audio::AudioCapture> = None;
            loop {
                if cap_tx_for_thread.is_closed() { break; }
                std::thread::sleep(std::time::Duration::from_millis(50));
                if cap.is_none() {
                    let mut c = kebi_audio::AudioCapture::new(16000);
                    let buf2 = buf_for_thread.clone();
                    if c.start(move |s| {
                        buf2.lock().extend_from_slice(s);
                    }).is_err() {
                        tracing::error!("audio start failed");
                        continue;
                    }
                    cap = Some(c);
                }
                while !stop_for_thread.load(std::sync::atomic::Ordering::SeqCst) {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
                if let Some(c) = cap.as_mut() {
                    let buf = c.buffer();
                    let mut b = buf.lock();
                    buf_for_thread.lock().append(&mut b);
                }
                let _ = cap_rx.try_recv();
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        let stop = stop_flag.clone();
        loop {
            if controller.state().get() != VoiceState::Listening {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                continue;
            }
            // Reset buffer for this turn.
            buffer.lock().clear();
            stop.store(false, std::sync::atomic::Ordering::SeqCst);
            let _ = cap_tx.send(());
            let _ = tx.send(VoiceEvent::State(VoiceState::Listening));

            while controller.state().get() == VoiceState::Listening {
                tokio::time::sleep(std::time::Duration::from_millis(80)).await;
            }
            stop.store(true, std::sync::atomic::Ordering::SeqCst);
            // Wait briefly for the capture thread to drain.
            tokio::time::sleep(std::time::Duration::from_millis(150)).await;

            let samples = { let mut b = buffer.lock(); std::mem::take(&mut *b) };
            if samples.is_empty() {
                let _ = tx.send(VoiceEvent::State(VoiceState::Idle));
                continue;
            }
            let _ = tx.send(VoiceEvent::State(VoiceState::Recognizing));

            let lang = config.general.language.clone();
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
                let _ = tx.send(VoiceEvent::State(VoiceState::Idle));
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
            let _ = tx.send(VoiceEvent::State(VoiceState::Idle));
        }
    });
}

async fn run_stt(config: &Config, samples: Vec<i16>, lang: &str) -> Result<String> {
    let api_key = config.get_stt_api_key()
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

    let lang = if config.general.language == "en" { "en" } else { "ru" };
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
            None => Command::Unknown { reason: "no_match".into() },
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
    let model = if config.llm.model.is_empty() { provider.default_model().to_string() } else { config.llm.model.clone() };
    let client = LlmClient::new(base, api_key, model, config.llm.timeout_secs);
    let messages = vec![
        kebi_llm::ChatMessage { role: "system".into(), content: kebi_llm::prompt::parse_system_prompt() },
        kebi_llm::ChatMessage { role: "user".into(), content: format!("Command: {text}") },
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
    if text.trim().is_empty() { return Ok(()); }
    let escaped = text.replace('\'', "''");
    let script = format!(
        "Add-Type -AssemblyName System.Speech; \
         $s = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
         $s.SelectVoice('{voice}') | Out-Null; \
         $s.Rate = {rate}; $s.Volume = {vol}; \
         $s.Speak('{escaped}')",
        voice = voice, rate = rate, vol = volume, escaped = escaped,
    );
    Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .output()?;
    Ok(())
}
