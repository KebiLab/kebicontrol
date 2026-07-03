# KebiControl

> Минималистичный голосовой ассистент для Windows.
> **Made by KebiLab**

KebiControl слушает микрофон, распознаёт речь (локально через Vosk или
облачно), понимает встроенные команды мгновенно, а если команда
нестандартная — отправляет запрос в LLM (OpenAI-совместимый endpoint,
по умолчанию **DeepSeek V4 Flash** через OpenCode Go) и исполняет
полученное действие.

## Возможности

- 🎤 Локальное распознавание речи (Vosk, ru+en) — без интернета
- ⚡ Встроенные команды: запуск/закрытие приложений, управление окнами,
  громкость, яркость, медиа-клавиши, ввод текста, скриншоты, таймеры
- 🤖 LLM-фолбэк: DeepSeek V4 Flash, MiniMax M3, MiMo V2.5, NVIDIA Nemotron
  или **свой OpenAI-совместимый провайдер**
- ⌨️ Глобальные горячие клавиши (по умолчанию `Ctrl+Shift+Space` —
  push-to-listen, `Ctrl+Shift+K` — оверлей-меню)
- 🪟 Оверлей-меню в стиле Win+G (минимализм, тёмная тема)
- 🔊 Голосовые ответы (Windows SAPI)
- 👤 Профили пользователей, журнал команд, автозапуск
- 🔐 API-ключи шифруются через Windows DPAPI

## Сборка

```powershell
git clone https://github.com/kebilab/kebicontrol.git
cd kebicontrol
cargo build --release
.\target\release\kebicontrol.exe
```

> Полная инструкция — в [docs/SETUP.md](docs/SETUP.md)

## Лицензия

Apache 2.0. См. [LICENSE](LICENSE).

**Made by KebiLab**
