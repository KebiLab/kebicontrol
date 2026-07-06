# KebiControl

Voice control for Windows. Made by KebiLab.

![logo](assets/logo/kebicontrol-logo.svg)

## How to use

1. Run `KebiControl.exe`.
2. The main window opens. Click the gear icon (top-right) to open Settings.
3. In Settings:
   - Pick a provider (OpenCode Go, OpenAI, Anthropic, Google Gemini, Mistral, Groq, DeepSeek, xAI, or your own).
   - Pick a model.
   - Paste your API key. Click the eye icon to show/hide. Click **Save and close**.
4. Back on the main window:
   - Type a command in the input and press Enter (or click ▶).
   - Or click a quick action: Pause, Screenshot, Quieter, Louder.
   - The status line under the buttons shows the result.
5. Switch language with the **Русский / English** button.
6. Switch theme with the **Тёмная / Светлая** button.

Your API key is stored encrypted with Windows DPAPI in
`%APPDATA%\KebiLab\KebiControl\config.toml`.

## Voice control

Voice recognition is not wired up in v0.1.0 yet — commands are typed
into the input field. Microphone + LLM STT and wake-word recognition
land in the next release.

## Build

```powershell
git clone https://github.com/KebiLab/kebicontrol.git
cd kebicontrol
cargo build --release
.\target\x86_64-pc-windows-gnu\release\kebicontrol.exe
```

## License

Apache 2.0. See LICENSE.

Made by KebiLab
