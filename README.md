# KebiControl

Voice control for Windows. Made by KebiLab.

KebiControl listens to your microphone, understands commands in Russian and English, and runs them locally. When a command is not built in, it asks a configured language model and executes the result.

## Features

- Local speech recognition (no internet required for built-in commands)
- Built-in commands: apps, windows, volume, brightness, media, input, timers, screenshots, power
- LLM fallback for anything else (configurable provider, OpenAI-compatible)
- Global hotkeys and an overlay panel
- Voice replies via Windows SAPI
- User profiles, command log, autostart on Windows login
- API keys encrypted with Windows DPAPI

## Build

```powershell
git clone https://github.com/KebiLab/kebicontrol.git
cd kebicontrol
cargo build --release
.\target\release\kebicontrol.exe
```

## License

Apache 2.0. See LICENSE.

Made by KebiLab
