# KebiControl

Voice control for Windows. Made by KebiLab.

![logo](assets/logo/kebicontrol-logo.svg)

## How to use

1. Run `KebiControl.exe`.
2. Click the gear icon (top-right) to open Settings.
3. In Settings:
   - **Распознавание речи (Whisper)** — paste your OpenAI key (or any Whisper-compatible endpoint) for speech-to-text.
   - **Нейросеть (LLM)** — pick a provider (OpenCode Go, OpenAI, Anthropic, Google Gemini, Mistral, Groq, DeepSeek, xAI, or your own), choose a model, paste the API key. Keys are encrypted with Windows DPAPI.
   - Click **Сохранить**.
4. Back on the main window:
   - Click the big **Говорить** button and speak. The app will listen, recognize, execute the command and answer with voice.
   - Or type a command in the input and press Enter.
   - Or click a quick action: Pause, Screenshot, Quieter, Louder.
5. Switch language with the **Русский / English** button.
6. Switch theme with the **Тёмная / Светлая** button.

Voice pipeline:

```
microphone (cpal, 16 kHz)
  -> STT (Whisper HTTP, your key)
  -> text
  -> local parser (regex rules) or LLM fallback
  -> Command
  -> action executor (Win32 / PowerShell)
  -> voice reply (SAPI via PowerShell)
```

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
