# Changelog

All notable changes to py2rs will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive library database with 300+ Python → Rust module mappings
- File selection interface with checkboxes (select/deselect specific files)
- Profile management system (save/load LLM provider configurations)
- Migration history tracking with detailed metrics
- Performance metrics display (tokens, cost estimates, duration per file)
- Drag & drop support for project folders
- .gitignore support (respect ignore patterns during file discovery)
- Dry-run mode (preview migration without writing files)
- Export functionality (JSON, CSV, Markdown reports)
- Webhook notifications for CI/CD integration
- Enhanced progress tracking with per-file timing
- Stop/resume functionality during migration
- GitHub Actions CI/CD workflows for automated testing and releases
- Multi-platform release builds (Windows MSI, macOS DMG, Linux AppImage)
- Comprehensive test suite (unit and integration tests)
- Example Python projects in `examples/` directory
- Architecture documentation (ARCHITECTURE.md)
- FAQ documentation (FAQ.md)
- Issue and PR templates
- Security policy (SECURITY.md)

### Changed
- Improved UI layout with better visual hierarchy
- Enhanced error reporting with detailed compilation feedback
- Better module documentation system with inline editing
- Optimized API call retry logic

## Версия 2.0 - Полный рефакторинг

### Новые функции

#### Автоматическая установка
- `setup.bat` - автоматически устанавливает все зависимости
- `start.bat` - запускает приложение одной командой
- Проверка наличия Rust и Node.js

#### Диалог первого запуска
- При первом запуске предлагает выбрать LLM провайдер
- Поддержка Ollama, llama.cpp, Anthropic
- Сохранение настроек между сессиями

#### Параллельная обработка
- Файлы без зависимостей обрабатываются параллельно
- Используется Rayon для многопоточности
- Значительно ускоряет миграцию больших проектов

#### Кэширование результатов
- MD5 хеш исходного кода + параметры
- Кэш сохраняется в `AppData/Local/py2rs/cache/`
- Пропуск повторной миграции идентичных файлов

#### Поддержка requirements.txt
- Автоматический парсинг `requirements.txt` и `pyproject.toml`
- Извлечение версий Python библиотек
- Отображение версий в UI для точного подбора крейтов

#### Улучшенный UI
- Прогресс-бар с процентами выполнения
- Счетчик [N/Total] для каждого файла
- Блокировка кнопки "Начать" во время работы
- Более читаемый вывод ошибок

#### Улучшенная обработка ошибок
- Структурированный разбор ошибок cargo check
- Категории ошибок: Missing import, Type mismatch, Trait requirement, Lifetime issue
- Более точный контекст для LLM при исправлении

### Изменения кода

#### Рефакторинг
- Удалены все AI-style комментарии и описания
- Код выглядит как написанный человеком
- Убраны избыточные пояснения
- Более лаконичные промпты для LLM

#### Новые модули
- `cache.rs` - кэширование переводов
- `parallel.rs` - параллельная обработка
- `provider.rs` - унифицированная работа с LLM
- `requirements.rs` - парсинг Python зависимостей

#### Оптимизации
- Использование Rayon для параллелизма
- Кэширование предотвращает повторные LLM запросы
- Более эффективная структура данных для графа зависимостей

### Исправления

- Исправлена обработка относительных импортов Python
- Улучшен парсинг `from ... import *`
- Корректная работа с пакетами через `__init__.py`
- Правильная обработка API ключей в keyring

### Документация

- `README.md` - краткое описание проекта
- `GUIDE.md` - подробное руководство по использованию
- `CHANGELOG.md` - список изменений
- Комментарии в коде оставлены только там, где необходимо

### Производительность

- Параллельная обработка: до 3-4x ускорение на многоядерных CPU
- Кэширование: мгновенная миграция уже обработанных файлов
- Оптимизация LLM запросов: меньше токенов, более точные промпты

---

## Как обновить

1. Удалите старую версию
2. Запустите `setup.bat`
3. При первом запуске выберите LLM провайдер

Кэш из предыдущих версий совместим и будет использован.
