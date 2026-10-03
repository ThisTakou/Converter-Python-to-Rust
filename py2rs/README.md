# py2rs — Python → Rust migration (Tauri 2)

Запуск:
    cargo install tauri-cli --version "^2"
    cargo tauri icon path/to/any-1024px.png   # иконки нужны для сборки
    cargo tauri dev

Провайдеры: любой OpenAI-совместимый endpoint (Ollama, LM Studio, OpenAI).
Результат пишется в `<папка проекта>_rs/src/`, исходник не трогается.


Готово: план по импортам, cargo check с автоисправлением, каталог из 139 модулей с документацией для ИИ + MD-окно для остальных, Anthropic API, ключ в keyring, финальная проверка проекта и REPORT.md.
Anthropic API, ключ в keyring, финальная проверка всего проекта и REPORT.md.

Возможные следующие шаги: параллельный перевод независимых файлов,
поддержка venv/requirements.txt для версий крейтов, редактор отчёта в UI.
