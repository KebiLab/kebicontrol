# Setup

Made by KebiLab.

## Requirements

- Windows 10 or 11 (x86_64)
- Rust 1.94+ (only if you build from source)
- A working microphone
- Internet connection only for: LLM requests and (optionally) cloud STT

## Quick start (prebuilt)

1. Download `KebiControl.exe` from the Releases page.
2. Place it anywhere on your disk.
3. Run it. On first launch it will:
   - create `%APPDATA%\KebiLab\KebiControl\config.toml`
   - show the overlay panel (or stay in the system tray)
4. Open Settings (Ctrl+Shift+K → Settings), paste your LLM API key, save.
5. Press and hold `Ctrl+Shift+Space` and speak a command.

## Build from source

```powershell
git clone https://github.com/KebiLab/kebicontrol.git
cd kebicontrol
cargo build --release
.\target\x86_64-pc-windows-gnu\release\kebicontrol.exe
```

The MinGW toolchain is bundled in `tools/mingw/`. The build uses
`x86_64-pc-windows-gnu` so no Visual Studio is required.

## LLM providers

`Settings → LLM` lets you pick a provider or define a custom one. Presets:

| Provider      | Base URL                              | Default model     |
|---------------|---------------------------------------|-------------------|
| OpenCode Go   | `https://api.opencode.ai/v1`          | `deepseek-v4-flash` |
| DeepSeek      | `https://api.deepseek.com/v1`         | `deepseek-v4-flash` |
| MiMo          | `https://api.xiaomimimo.com/v1`       | `mimo-v2.5`       |
| NVIDIA        | `https://integrate.api.nvidia.com/v1`  | `nvidia/nemotron` |
| Custom        | your endpoint                         | your model        |

The API key field is masked. On save the key is encrypted with Windows DPAPI
and stored in `config.toml` as base64. The plaintext key never touches disk.

## File locations

- Config: `%APPDATA%\KebiLab\KebiControl\config.toml`
- Profiles: `%APPDATA%\KebiLab\KebiControl\profiles\`
- Logs: `%LOCALAPPDATA%\KebiLab\KebiControl\cache\logs\`

## Autostart

Settings → General → "Launch with Windows" creates a shortcut in your
Startup folder (no registry edits).

## License

Apache 2.0. See LICENSE.
