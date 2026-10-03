# Architecture Documentation

## Overview

py2rs is a **Tauri 2.0 desktop application** that uses LLMs to migrate Python projects to Rust. The architecture follows a clear separation between backend (Rust) and frontend (HTML/CSS/JS), communicating via Tauri's IPC bridge.

```
┌─────────────────────────────────────────────────────────────┐
│                         Frontend (UI)                        │
│  HTML/CSS/JS + i18n system + localStorage state management  │
└──────────────────────┬──────────────────────────────────────┘
                       │ Tauri IPC Commands
┌──────────────────────▼──────────────────────────────────────┐
│                      Backend (Rust)                          │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────────┐   │
│  │   main.rs   │  │  api_call.rs │  │  crate_map.rs    │   │
│  │  (commands) │  │  (LLM calls) │  │  (300+ mappings) │   │
│  └─────────────┘  └──────────────┘  └──────────────────┘   │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────────┐   │
│  │  check.rs   │  │  lib.rs      │  │  model config    │   │
│  │(cargo check)│  │  (exports)   │  │  in tauri.conf   │   │
│  └─────────────┘  └──────────────┘  └──────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Project Structure

```
py2rs/
├── ui/                          # Frontend
│   ├── index.html               # Main UI structure
│   ├── app.js                   # Core logic, IPC calls, state management
│   ├── style.css                # Styling
│   └── i18n/                    # Internationalization
│       ├── ru.json              # Russian translations
│       └── en.json              # English translations
├── py2rs/                       # Tauri app
│   ├── src-tauri/               # Rust backend
│   │   ├── src/
│   │   │   ├── main.rs          # Tauri commands, IPC handlers
│   │   │   ├── api_call.rs      # LLM provider implementations
│   │   │   ├── crate_map.rs     # Python→Rust library database
│   │   │   ├── check.rs         # Cargo compilation checking
│   │   │   └── lib.rs           # Module exports
│   │   ├── Cargo.toml           # Rust dependencies
│   │   └── tauri.conf.json      # Tauri configuration
│   └── target/                  # Build artifacts
├── examples/                    # Demo Python projects
├── .github/                     # CI/CD workflows
│   └── workflows/
│       ├── ci.yml               # Test on push/PR
│       └── release.yml          # Build releases
├── README.md                    # Main documentation
├── CONTRIBUTING.md              # Contribution guide
├── CODE_OF_CONDUCT.md           # Community standards
├── SECURITY.md                  # Security policy
└── ARCHITECTURE.md              # This file
```

## Core Components

### 1. Frontend (ui/)

**index.html** — UI structure with sections:
- Provider configuration (OpenAI, Anthropic, llama.cpp, Custom API)
- Project selection and file preview
- Documentation editor (Markdown)
- Progress display and results
- Profile management dialog
- Migration history dialog

**app.js** — Frontend logic:
- `loadTranslations()` — i18n system
- `switchKind()` — Toggle between LLM providers
- `pickDir()` — Tauri IPC call to select Python project
- `start()` — Main migration loop with cancellation support
- Profile save/load — localStorage-based configuration management
- History tracking — localStorage-based migration history
- File selection — Set-based file filtering

**style.css** — Modern, clean UI styling with:
- CSS variables for theming
- Responsive layouts
- Dialog styles
- Progress animations

**i18n/** — Translation files with key-value pairs for all UI strings

### 2. Backend (py2rs/src-tauri/src/)

**main.rs** — Tauri commands exposed to frontend:
```rust
#[tauri::command]
async fn pick_python_dir() -> Result<(String, Vec<FileEntry>), String>
// Scans directory for .py files

#[tauri::command]
async fn migrate_one(py_code: String, docs: String, libs_known: Vec<(String, String, String)>) -> Result<String, String>
// Calls LLM to migrate single file

#[tauri::command]
async fn cargo_check(rs_code: String, tmp_dir: String) -> Result<CheckResult, String>
// Runs cargo check on generated Rust code
```

**api_call.rs** — LLM provider abstraction:
```rust
pub enum ApiKind {
    OpenAI,
    Anthropic,
    LlamaCpp,
    Custom { response_path, body_template, headers },
}

pub async fn call_llm(kind: &ApiKind, base_url: &str, model: &str, api_key: Option<&str>, user_message: &str) -> Result<String>
```

Each provider has its own request/response format handling. Adding a new provider means:
1. Add variant to `ApiKind` enum
2. Implement HTTP request formatting in `call_llm()`
3. Parse provider-specific response structure
4. Add UI fields in `index.html` if needed

**crate_map.rs** — Python→Rust library mapping database:
```rust
pub struct LibInfo {
    pub crate_hint: &'static str,  // Rust crate name
    pub doc: &'static str,          // LLM guidance documentation
}

pub fn catalog() -> HashMap<&'static str, LibInfo>
```

Contains 300+ pre-loaded mappings like:
```rust
lib!("requests", "reqwest", "Simple HTTP client for GET/POST calls and REST APIs...")
lib!("pandas", "polars", "DataFrames for tabular data analysis...")
lib!("flask", "axum", "Lightweight web framework for routes and handlers...")
```

**check.rs** — Cargo integration:
- Creates temporary project with generated Rust code
- Runs `cargo check` to validate syntax and dependencies
- Parses compiler output for errors/warnings
- Returns structured result to frontend

**lib.rs** — Module exports for the backend

### 3. Configuration

**tauri.conf.json** — Tauri app configuration:
- Window size and title
- Bundle identifier
- Icon paths
- Build settings
- Allowed IPC commands

**Cargo.toml** — Rust dependencies:
- `tauri` — Desktop app framework
- `reqwest` — HTTP client for LLM APIs
- `serde`, `serde_json` — Serialization
- `tokio` — Async runtime

## Data Flow

### Migration Flow

1. **User selects Python project** → Frontend calls `pick_python_dir()` → Backend scans for `.py` files → Returns file list
2. **User configures LLM provider** → Frontend stores in profile (localStorage)
3. **User writes documentation** → Markdown editor (instructions for LLM)
4. **User clicks "Начать"** → Frontend loop:
   - For each selected file:
     - Call `migrate_one()` with Python code, docs, known libraries
     - Backend calls LLM via `api_call.rs`
     - LLM returns Rust code
     - Call `cargo_check()` to validate
     - Display result (success/failure) in UI
     - Update progress bar
5. **Save results** → Frontend stores in migration history (localStorage)

### LLM Call Flow

```
Frontend (app.js)
    ↓ invoke("migrate_one", {...})
Backend (main.rs::migrate_one)
    ↓ call_llm(kind, base_url, model, api_key, prompt)
api_call.rs
    ↓ HTTP POST to provider endpoint
LLM Provider (OpenAI/Anthropic/local)
    ↓ Response with Rust code
api_call.rs (parse response)
    ↓ Extract code from JSON
Backend (main.rs)
    ↓ Return Rust code string
Frontend (app.js)
    ↓ Display result
```

## Extensibility Guide

### Adding a New LLM Provider

**Example: Adding Ollama support**

1. **Update `api_call.rs`:**

```rust
pub enum ApiKind {
    OpenAI,
    Anthropic,
    LlamaCpp,
    Ollama,  // NEW
    Custom { ... },
}

pub async fn call_llm(...) -> Result<String> {
    match kind {
        // ... existing providers ...
        
        ApiKind::Ollama => {
            let body = json!({
                "model": model,
                "prompt": user_message,
                "stream": false,
            });
            
            let resp = client.post(format!("{}/api/generate", base_url))
                .json(&body)
                .send()
                .await?;
            
            let data: serde_json::Value = resp.json().await?;
            let text = data["response"].as_str()
                .ok_or("No response field")?;
            Ok(text.to_string())
        }
    }
}
```

2. **Update `index.html`:**

```html
<select id="kind" onchange="switchKind()">
    <option value="openai">OpenAI (GPT-4, etc.)</option>
    <option value="anthropic">Anthropic (Claude)</option>
    <option value="llamacpp">llama.cpp</option>
    <option value="ollama">Ollama (local)</option>
    <option value="custom">Custom API</option>
</select>
```

3. **Update `app.js`:**

```javascript
function switchKind() {
    const k = $("kind").value;
    // ... existing logic ...
    
    if (k === "ollama") {
        show("baseRow");
        show("modelRow");
        hide("keyRow");
        $("base").value = "http://localhost:11434";
        $("model").value = "llama2";
    }
}

// In start() function, map to backend enum:
async function start() {
    const kind = $("kind").value;  // "ollama"
    // Tauri command will receive "ollama" and map to ApiKind::Ollama
}
```

4. **Update translations** in `ui/i18n/ru.json` and `en.json` if needed

5. **Test** with a local Ollama instance

### Expanding the Library Database

The `crate_map.rs` file contains 300+ Python→Rust library mappings. To add more:

**Format:**
```rust
lib!("python_module", "rust_crate", "Documentation for the LLM explaining the Rust equivalent")
```

**Guidelines:**
- **Documentation** should help the LLM understand the migration context
- Mention key API differences (sync vs async, blocking vs non-blocking)
- Reference similar functions or patterns where applicable
- Include caveats ("no direct equivalent, use X + Y instead")

**Example:**
```rust
lib!("beautifulsoup4", "scraper", 
     "Parse HTML and query it with CSS selectors. scraper does the same using CSS selectors over a parsed DOM. Use Html::parse_document() then select() with CSS syntax."),
```

**Special cases:**
- **No Rust equivalent:** Explain why (e.g., "Rust has no GIL, so no multiprocessing needed")
- **Multiple crates needed:** List them with "+" (e.g., "tokio + tokio-tungstenite")
- **Built-in Rust feature:** Reference std lib (e.g., "std::fs for file operations")

### Customizing the UI

**Adding a new feature to the UI:**

1. **Add HTML structure** in `index.html`:
```html
<div id="myFeatureRow" class="row">
    <label for="myFeature" data-t="myFeatureLabel"></label>
    <input type="text" id="myFeature" placeholder="...">
</div>
```

2. **Add translations** in `ui/i18n/*.json`:
```json
{
    "myFeatureLabel": "My Feature:",
    "myFeaturePlaceholder": "Enter value..."
}
```

3. **Add JavaScript logic** in `app.js`:
```javascript
function handleMyFeature() {
    const value = $("myFeature").value;
    // Do something with value
}
```

4. **Call from backend** if needed via Tauri command:
```javascript
const result = await invoke("my_backend_command", { value });
```

5. **Implement backend command** in `main.rs`:
```rust
#[tauri::command]
async fn my_backend_command(value: String) -> Result<String, String> {
    // Implementation
    Ok("success".into())
}
```

### Adding Migration Tests

Tests ensure generated code is valid Rust. Current system uses `cargo check`.

**To add more validation:**

1. **Extend `check.rs`:**
```rust
pub async fn cargo_test(rs_code: String, tmp_dir: String) -> Result<TestResult, String> {
    // Run cargo test instead of just cargo check
    let output = Command::new("cargo")
        .args(&["test", "--manifest-path", &manifest_path])
        .output()?;
    
    // Parse test results
    // Return structured data
}
```

2. **Add Tauri command** in `main.rs`:
```rust
#[tauri::command]
async fn run_tests(rs_code: String, tmp_dir: String) -> Result<TestResult, String> {
    check::cargo_test(rs_code, tmp_dir).await
}
```

3. **Call from frontend** in `app.js`:
```javascript
const testResult = await invoke("run_tests", { 
    rsCode: generatedCode,
    tmpDir: "/tmp/py2rs_test"
});
```

### State Management

**Frontend state** (stored in `localStorage`):
- `profiles` — Array of saved LLM configurations
- `migrationHistory` — Array of past migration runs (limited to 50)
- UI preferences can be added similarly

**Backend state:**
- Currently stateless between commands
- To add persistence, use `tauri::State` with Arc<Mutex<...>>

**Example of backend state:**
```rust
struct AppState {
    cache: HashMap<String, String>,
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            cache: HashMap::new(),
        })
        .invoke_handler(tauri::generate_handler![my_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn my_command(state: tauri::State<AppState>) -> String {
    // Access shared state
    state.cache.get("key").cloned().unwrap_or_default()
}
```

## Development Workflow

### Local Development

```bash
# Install dependencies
cd py2rs
npm install

# Run in development mode (hot reload)
npm run tauri dev

# Format Rust code
cargo fmt

# Lint Rust code
cargo clippy

# Run Rust tests
cargo test
```

### Building for Release

```bash
# Build for current platform
npm run tauri build

# Output:
# - Windows: .msi installer in src-tauri/target/release/bundle/msi/
# - macOS: .dmg in src-tauri/target/release/bundle/dmg/
# - Linux: .AppImage in src-tauri/target/release/bundle/appimage/
```

### Automated Builds

GitHub Actions workflows in `.github/workflows/`:

- **ci.yml** — Runs on every push/PR:
  - Cargo fmt check
  - Cargo clippy
  - Cargo test
  - npm build
  - Multi-platform matrix (Ubuntu, Windows, macOS)

- **release.yml** — Runs on version tags (v*):
  - Builds for all platforms (x86_64 + aarch64)
  - Creates GitHub release
  - Uploads installers as release assets

## Security Considerations

1. **API Keys** — Stored in system credential manager (not localStorage):
   - Windows: Windows Credential Manager
   - macOS: Keychain
   - Linux: Secret Service API

2. **LLM Calls** — All use HTTPS/TLS

3. **Generated Code** — Always review before using in production
   - Run `cargo clippy` on output
   - Check for unsafe blocks
   - Validate error handling

4. **Sandboxing** — Tauri provides OS-level sandboxing

5. **IPC** — Only explicitly allowed commands can be called from frontend

See [SECURITY.md](SECURITY.md) for detailed security policy.

## Performance Optimization

### Current Bottlenecks

1. **Sequential migration** — Files are migrated one by one
2. **LLM latency** — Network calls to external APIs
3. **Cargo check** — Spawning process per file

### Optimization Ideas

**Parallel migration:**
```javascript
// In app.js, migrate multiple files concurrently
async function startParallel() {
    const concurrency = 3;
    const queue = selectedFiles.slice();
    const workers = Array(concurrency).fill(null).map(async () => {
        while (queue.length > 0) {
            const file = queue.shift();
            await migrateFile(file);
        }
    });
    await Promise.all(workers);
}
```

**Caching LLM responses:**
```rust
// In api_call.rs, add caching layer
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static! {
    static ref CACHE: Arc<Mutex<HashMap<String, String>>> = 
        Arc::new(Mutex::new(HashMap::new()));
}

pub async fn call_llm_cached(...) -> Result<String> {
    let cache_key = format!("{:?}:{}", kind, user_message);
    
    let mut cache = CACHE.lock().await;
    if let Some(cached) = cache.get(&cache_key) {
        return Ok(cached.clone());
    }
    drop(cache);
    
    let result = call_llm(...).await?;
    
    CACHE.lock().await.insert(cache_key, result.clone());
    Ok(result)
}
```

**Batch cargo check:**
```rust
// Check multiple files in one temporary project
pub async fn cargo_check_batch(files: Vec<(String, String)>) -> Result<Vec<CheckResult>> {
    // Create project with multiple files in src/
    // Run cargo check once
    // Parse per-file errors
}
```

## Troubleshooting

### Common Issues

**"Workspace unavailable"** — Linux VM failed to start:
- Ensure virtualization is enabled in BIOS
- On Windows, enable Hyper-V or WSL2

**"Failed to call LLM"** — Network or API key issue:
- Check API key is correct
- Verify base URL is reachable
- Check firewall settings

**"Cargo check failed"** — Generated code has errors:
- Review LLM output in UI
- Check if required crates are in Cargo.toml
- Improve documentation provided to LLM

**High memory usage** — Large projects:
- Reduce concurrency in parallel migration
- Clear migration history periodically
- Use file selection to migrate in batches

### Debug Mode

To enable Tauri dev tools:

```javascript
// In app.js, add debug logging
const DEBUG = true;

function debugLog(...args) {
    if (DEBUG) console.log("[DEBUG]", ...args);
}

// Use in code
debugLog("Calling migrate_one with", { pyCode, docs });
```

Tauri console is accessible in dev mode via Ctrl+Shift+I (or Cmd+Opt+I on macOS).

## Future Enhancements

Potential features for future development:

1. **Incremental migration** — Track partially migrated projects
2. **Diff viewer** — Side-by-side Python vs Rust code
3. **Test generation** — Ask LLM to generate Rust tests
4. **Dependency resolver** — Auto-add crates to Cargo.toml
5. **Cost tracking** — Calculate API costs per migration
6. **Quality metrics** — Rate migration quality (idiomatic Rust score)
7. **CLI version** — Command-line interface for CI/CD integration
8. **Plugin system** — User-defined pre/post-processing hooks
9. **Export formats** — JSON/CSV reports for analysis
10. **Webhook notifications** — Alert on migration completion

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to contribute these features.

## License

[Your license here - likely MIT or Apache 2.0]

---

**Questions?** Open an issue on GitHub or check the [README](README.md) for general usage.
