# Architecture

KebiControl — voice control for Windows. Made by KebiLab.

## Stack

- Rust 1.94+ (stable)
- GUI: eframe / egui
- Audio: cpal
- STT: Vosk (optional, local) or Whisper-compatible HTTP API
- LLM: any OpenAI-compatible endpoint (OpenCode Go by default)
- TTS: Windows SAPI via PowerShell
- Win32: windows crate 0.61
- MinGW 16.1 (GNU toolchain)

## Crates

```
KebiControl/
  crates/
    kebi-core/     Command enum, config, profiles, parser (local + LLM)
    kebi-audio/    cpal capture + RMS VAD
    kebi-stt/      Vosk + Whisper HTTP
    kebi-llm/      OpenAI-compatible client with provider presets
    kebi-tts/      TTS via SAPI (PowerShell bridge)
    kebi-ui/       egui overlay + settings + theme
  app/             binary
  tools/
    mingw/         GNU toolchain (bundled)
    upx/           executable compressor
```

## Data flow

```
microphone
  -> cpal (16 kHz mono)
  -> VAD (silence detection)
  -> STT (Vosk local OR Whisper HTTP)
  -> text
  -> Parser
       local rules (regex)
       OR LLM (OpenCode Go / DeepSeek / etc.)
  -> Command
  -> Action executor (Win32 / enigo / PowerShell)
  -> optional TTS reply
```

## Security

- API keys are stored in `config.toml` encrypted with Windows DPAPI
  (`CryptProtectData` with `CRYPTPROTECT_LOCAL_MACHINE`).
- The config file lives at `%APPDATA%\KebiLab\KebiControl\config.toml`.
- Plaintext keys are never written to disk.

## Config

- `config/config.example.toml` — annotated reference
- `config.toml` — actual user config (auto-created on first run)

## License

Apache 2.0. See LICENSE.
