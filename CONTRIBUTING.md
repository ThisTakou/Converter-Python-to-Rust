# Contributing to py2rs

Thank you for your interest in contributing to py2rs! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/py2rs.git`
3. Create a feature branch: `git checkout -b feature/your-feature-name`

## Development Setup

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Node.js 16+ (for UI development)
- Tauri CLI: `cargo install tauri-cli`

### Building the Project

```bash
cd py2rs
npm install
cargo tauri dev
```

## Project Structure

```
py2rs/
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── main.rs     # Entry point
│   │   ├── migrate.rs  # Migration logic
│   │   └── llm.rs      # LLM provider integration
│   └── Cargo.toml
├── ui/                 # Frontend
│   ├── index.html
│   ├── app.js
│   ├── style.css
│   └── i18n/          # Translations
└── README.md
```

## Code Guidelines

### Rust Code

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Add tests for new functionality

### JavaScript Code

- Use modern ES6+ syntax
- Keep functions small and focused
- Comment complex logic
- Test UI changes in both light and dark themes

### Internationalization

When adding new UI text:
1. Add keys to `ui/i18n/en.json`
2. Add corresponding translations to `ui/i18n/ru.json`
3. Use `data-t` attribute in HTML or access via `T` object in JS

## Submitting Changes

1. **Test your changes**: Ensure the app builds and runs without errors
2. **Write clear commit messages**: 
   - Use present tense: "Add feature" not "Added feature"
   - Keep first line under 72 characters
   - Reference issues: "Fix #123: Handle empty Python files"
3. **Update documentation**: If you change functionality, update README.md
4. **Push to your fork**: `git push origin feature/your-feature-name`
5. **Create a Pull Request**: Describe what you changed and why

## Pull Request Guidelines

- **One feature per PR**: Keep changes focused and atomic
- **Describe the change**: Explain what you changed, why, and how to test it
- **Link related issues**: Use "Fixes #123" or "Closes #456"
- **Be responsive**: Address review feedback promptly

## Reporting Bugs

When reporting bugs, please include:

- **OS and version** (Windows 11, macOS 14, Ubuntu 22.04, etc.)
- **py2rs version** (from About dialog or `Cargo.toml`)
- **LLM provider** (OpenAI, Anthropic, local model, etc.)
- **Steps to reproduce**
- **Expected vs actual behavior**
- **Error messages or logs**

## Feature Requests

We welcome feature requests! Please:

- Check existing issues first to avoid duplicates
- Describe the use case clearly
- Explain how it would help you and others
- Consider whether it fits the project's scope

## Adding LLM Providers

To add support for a new LLM provider:

1. Add the provider enum variant in `src-tauri/src/llm.rs`
2. Implement the API call logic in the `call_llm` function
3. Add UI options in `ui/index.html` and `ui/app.js`
4. Update translations in `ui/i18n/*.json`
5. Document the provider in README.md

## Code Review Process

- Maintainers will review your PR within 7 days
- Address feedback by pushing new commits to your branch
- Once approved, a maintainer will merge your PR
- Your contribution will be credited in the release notes

## Questions?

If you have questions about contributing, feel free to:

- Open an issue with the "question" label
- Check existing discussions
- Reach out to maintainers

Thank you for making py2rs better! 🚀
