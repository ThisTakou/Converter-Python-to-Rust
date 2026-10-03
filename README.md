# py2rs - Python to Rust Converter

**Intelligent Python to Rust migration powered by LLMs**

py2rs is a desktop application that uses Large Language Models to automatically migrate Python projects to Rust. It analyzes your codebase, detects dependencies, and generates idiomatic Rust code with compile checks and iterative refinement.

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ThisTakou/Converter-Python-to-Rust/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/tauri-2.0-brightgreen.svg)](https://tauri.app/)
[![GitHub Issues](https://img.shields.io/github/issues/ThisTakou/Converter-Python-to-Rust)](https://github.com/ThisTakou/Converter-Python-to-Rust/issues)
[![GitHub Stars](https://img.shields.io/github/stars/ThisTakou/Converter-Python-to-Rust?style=social)](https://github.com/ThisTakou/Converter-Python-to-Rust/stargazers)

## Features

### Core Features
- **LLM-Powered Migration** - Uses AI to understand Python code and generate equivalent Rust
- **1000+ Library Mappings** - Comprehensive database of Python → Rust crate equivalents (auto-updated every 3 days)
- **Automatic Dependency Detection** - Identifies external libraries and suggests Rust crates
- **Compile Verification** - Checks generated code with `cargo check` and iterates on errors
- **Smart Context** - Analyzes project structure and maintains consistency across files

### LLM Support
- **Multiple LLM Providers** - OpenAI, Anthropic, Ollama, llama.cpp, Groq, Azure OpenAI, custom APIs
- **Profile Management** - Save and load different LLM configurations
- **Cost Tracking** - Monitor token usage and estimated costs
- **Performance Metrics** - Track migration speed, success rates, and API usage

### User Experience
- **File Selection** - Choose specific files to migrate with checkboxes
- **Quality Levels** - Fast/Balanced/Strict migration modes
- **Test Generation** - Automatically create unit tests for migrated code
- **README Generation** - Create documentation for Rust projects
- **Multilingual UI** - English and Russian interfaces
- **Parallel Processing** - Configurable thread count for faster migrations
- **Dark Mode** - Automatic dark theme support
- **Stop/Resume** - Pause and resume long-running migrations

### Advanced Features
- **Dry-Run Mode** - Preview migration without writing files
- **.gitignore Support** - Respect ignore patterns during file discovery
- **Migration History** - Track all past migrations with detailed reports
- **Export Options** - JSON, CSV, and Markdown report formats
- **Webhook Notifications** - CI/CD integration with custom endpoints
- **Drag & Drop** - Drop folders directly into the app

## Quick Start

### Option 1: Download Pre-built Release (Recommended)

1. Go to [Releases](https://github.com/ThisTakou/Converter-Python-to-Rust/releases)
2. Download the latest version for your platform:
   - **Windows**: `py2rs-windows-x64.msi`
   - **macOS**: `py2rs-macos-universal.dmg`
   - **Linux**: `py2rs-linux-x64.AppImage`
3. Install and run the application
4. Follow the first-run setup wizard to configure your LLM provider

### Option 2: Build from Source

#### Windows

1. Run `setup.bat` - installs dependencies and creates icons
   - If antivirus blocks: run as Administrator or add exception for project folder
   - Alternative: `setup-quick.bat` for faster debug build
2. Run `start.bat` - opens the application
3. Choose your Python project folder
4. Configure LLM provider
5. Click "Start migration"

#### Linux/macOS

```bash
# Clone the repository
git clone https://github.com/ThisTakou/Converter-Python-to-Rust.git
cd Converter-Python-to-Rust/py2rs

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Requirements

- [Rust](https://rustup.rs/) 1.70 or later
- [Node.js](https://nodejs.org/) 16 or later
- API key for LLM or local llama.cpp/Ollama server

## Configuration

### First Run Setup

On first launch, configure an LLM provider:

- **Provider Type**: Choose from OpenAI-compatible, Anthropic, llama.cpp, or Custom API
- **Base URL**: API endpoint
- **API Key**: Your API key (optional for local models)
- **Model**: Model name

### Supported LLM Providers

#### OpenAI-Compatible APIs

```
Provider: Local / OpenAI-compatible
Base URL: https://api.openai.com/v1
API Key: sk-...
Model: gpt-4-turbo
```

Also supports:
- **Ollama**: `http://localhost:11434/v1` (no API key)
- **router.cheap**: `https://router.cheap/v1` or `https://direct.router-cheap.com/v1`
- **LM Studio**: `http://localhost:1234/v1`
- **LocalAI**: Your local instance URL

#### Anthropic API

```
Provider: Anthropic API
API Key: sk-ant-...
Model: claude-3-5-sonnet-20241022
```

#### llama.cpp Server

Run `llama.cpp` with `--server` flag, then:

```
Provider: llama.cpp
Base URL: http://localhost:8080
Model: (leave default)
```

## Usage

### Basic Workflow

1. **Select Project**: Click "Choose project folder" and select your Python project root
2. **Project Description**: Describe your project in 2-3 sentences (helps AI understand context)
3. **Review Libraries**: Check and edit suggested Rust crates for detected Python dependencies
4. **Select Files**: Use checkboxes to choose which files to migrate (or select all)
5. **Configure Quality**: Choose migration quality level:
   - **Fast/Lenient**: Quick migration, may need manual fixes
   - **Balanced**: Optimal for most projects (default)
   - **Strict**: Maximum quality, slower
6. **Optional Settings**:
   - Generate tests for migrated code
   - Create README.md for Rust project
   - Dry-run mode (preview without writing files)
   - Respect .gitignore patterns
7. **Add Documentation**: Click the doc button to add notes for specific modules
8. **Start Migration**: Click "Start migration" and monitor progress
9. **Review Results**: Check generated `*_rs/` directory and compilation report

### Advanced Features

#### Profile Management
- Save LLM configurations for quick switching
- Load saved profiles with one click
- Manage multiple API keys for different providers

#### Migration History
- View all past migrations
- Track success rates and performance metrics
- Export detailed reports (JSON, CSV, Markdown)

#### Export & Automation
- Export migration reports in multiple formats
- Configure webhook notifications for CI/CD
- Integrate with Slack, Discord, or custom endpoints

### Settings

- **Thread Count**: Number of parallel LLM requests (1-16, default: 4)
- **Dark Theme**: Toggle dark/light mode
- **Webhook URL**: POST migration results to external services
- **Language**: Switch between English and Russian

## Project Structure

```
py2rs/
├── setup.bat               # Windows setup script
├── start.bat               # Windows launcher
├── py2rs/
│   ├── ui/                 # Frontend (HTML/CSS/JS)
│   │   ├── index.html
│   │   ├── app.js
│   │   ├── style.css
│   │   └── i18n/           # Translations (en, ru)
│   └── src-tauri/          # Rust backend (Tauri)
│       └── src/
│           ├── main.rs     # Entry point and Tauri commands
│           ├── migrate.rs  # Migration orchestration
│           ├── crate_map.rs # Python → Rust library mappings
│           └── llm.rs      # LLM provider integrations
├── examples/               # Example Python projects
│   ├── simple_calculator/
│   ├── flask_api/
│   └── data_processing/
├── .github/                # CI/CD workflows and templates
├── LICENSE                 # MIT License
├── README.md               # This file
├── CONTRIBUTING.md         # Contribution guidelines
├── ARCHITECTURE.md         # System architecture documentation
├── FAQ.md                  # Frequently asked questions
└── CHANGELOG.md            # Version history
```

## How It Works

1. **Project Analysis**: Scans Python files, detects imports, builds dependency graph
2. **Library Detection**: Identifies external libraries and suggests Rust crates from 300+ pre-loaded mappings
3. **File-by-File Migration**: Processes files in dependency order
   - Sends Python code + context to LLM
   - Receives generated Rust code
   - Runs `cargo check` with parallel thread limit
   - Iterates on compilation errors (up to 5 attempts)
4. **Interactive Questions**: Asks user for clarification when needed
5. **Final Verification**: Builds entire Rust project and generates detailed report

## Tips for Best Results

- **Clear Description**: A good 2-3 sentence project description helps AI understand purpose and constraints
- **Document Modules**: Add documentation for complex or domain-specific modules
- **Choose Right Model**: 
  - Larger models (GPT-4, Claude Opus) - better quality, slower, more expensive
  - Smaller models (GPT-3.5, local models) - faster, cheaper, may need more iterations
- **Review Crates**: AI suggests Rust equivalents, but you can override them
- **Manual Review**: Always review generated code - AI migration is not perfect
- **Start Small**: Test with a small module first to verify configuration

## Limitations

- **Not 100% Accurate**: Generated code may need manual fixes
- **Python-Specific Features**: Some Python idioms don't translate directly to Rust
- **Dynamic Typing**: Type inference is best-effort; complex types may need adjustment
- **External Dependencies**: Quality depends on how well AI knows the suggested crates
- **API Costs**: Large projects can consume significant API credits

## Troubleshooting

### 403 Forbidden Error

If you see `HTTP status client error (403 Forbidden)`:
- **Check API key**: Ensure it's valid and has credits
- **Verify endpoint**: For router.cheap, use OpenAI-compatible provider (not Custom API)
- **Try backup URL**: Use `https://direct.router-cheap.com/v1` if main endpoint fails

### Compilation Errors

If many files fail to compile:
- Reduce thread count (Settings) to avoid overwhelming `cargo check`
- Add more context in project description
- Document complex modules
- Try a more capable model
- Use stricter quality level

### Slow Performance

- Increase thread count (Settings)
- Use faster/cheaper models for initial pass
- Use local models (Ollama) for no API latency
- Enable dry-run mode to test before actual migration

### Antivirus Blocking (Windows)

- Run `setup.bat` as Administrator
- Add exception for `py2rs` folder
- Use `setup-quick.bat` (debug build, faster)

## Documentation

- **[FAQ](FAQ.md)**: Frequently asked questions
- **[CONTRIBUTING](CONTRIBUTING.md)**: How to contribute to the project
- **[ARCHITECTURE](ARCHITECTURE.md)**: System design and architecture
- **[CHANGELOG](CHANGELOG.md)**: Version history and roadmap
- **[Examples](examples/)**: Sample Python projects for testing

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Ways to contribute:
- Report bugs and request features via [GitHub Issues](https://github.com/ThisTakou/Converter-Python-to-Rust/issues)
- Submit pull requests with improvements
- Add new Python → Rust library mappings
- Improve documentation and examples
- Share your migration success stories

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app/) for cross-platform desktop apps
- Powered by Large Language Models from OpenAI, Anthropic, and the open-source community
- Inspired by the need for safer, faster systems programming

## Support

- **Issues**: [GitHub Issues](https://github.com/ThisTakou/Converter-Python-to-Rust/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ThisTakou/Converter-Python-to-Rust/discussions)
- **FAQ**: [FAQ.md](FAQ.md)

---

**Made with care for developers migrating to Rust**
