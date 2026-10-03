mod anthropic;
mod cache;
mod check;
mod crate_map;
mod graph;
mod parallel;
mod provider;
mod requirements;
#[cfg(test)]
mod tests;
use provider::Provider;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fs, path::PathBuf};

const MAX_FIX: usize = 4;

#[tauri::command]
fn plan_project(root: String) -> Vec<graph::PlanItem> {
    graph::plan(&PathBuf::from(root))
}

#[derive(Serialize)]
struct LibSuggestion {
    py: String,
    suggestion: Option<String>,
    doc: Option<String>,
}

#[tauri::command]
fn detect_libs(root: String) -> Vec<LibSuggestion> {
    let catalog = crate_map::catalog();
    let root_path = PathBuf::from(&root);
    let py_deps = requirements::parse_requirements(&root_path);

    graph::external_modules(&root_path)
        .into_iter()
        .map(|py| {
            let info = catalog.get(py.as_str());
            let mut suggestion = info.map(|i| i.crate_hint.to_string());

            if let Some(version) = py_deps.get(&py.to_lowercase()) {
                if let Some(ref mut s) = suggestion {
                    if !version.is_empty() {
                        *s = format!("{} (Python v{})", s, version);
                    }
                }
            }

            LibSuggestion {
                suggestion,
                doc: info.map(|i| i.doc.to_string()),
                py,
            }
        })
        .collect()
}

#[derive(Deserialize)]
struct MigrateArgs {
    root: String,
    file: String,
    description: String,
    lang: String,
    deps: Vec<String>,
    crates: HashMap<String, String>,
    docs: HashMap<String, String>,
    answers: Vec<String>,
    provider: Provider,
    thread_count: Option<usize>,
}

#[derive(Serialize)]
#[serde(tag = "kind")]
enum Outcome {
    Ask { question: String },
    Done { rust: String, out_path: String, checked: bool, errors: String },
}

fn system_prompt(desc: &str, lang: &str) -> String {
    format!(
        "Migrate Python code to Rust.\nProject: {desc}\n\
         Library docs below.\n\
         If clarification needed, reply with JSON {{\"ask\":\"<question in {lang}>\"}}.\n\
         Otherwise reply with Rust source only, no explanations.\n\
         For external crates, add `// deps: crate1, crate2` as first line."
    )
}

fn strip_fence(t: &str) -> String {
    t.trim().trim_start_matches("```rust").trim_end_matches("```").trim().to_string()
}

fn parse_ask(text: &str) -> Option<String> {
    let t = text.trim().trim_start_matches("```json").trim_end_matches("```").trim();
    serde_json::from_str::<serde_json::Value>(t).ok()?["ask"].as_str().map(String::from)
}

#[tauri::command]
async fn migrate_file(a: MigrateArgs) -> Result<Outcome, String> {
    let root = PathBuf::from(&a.root);
    let src = fs::read_to_string(root.join(&a.file)).map_err(|e| e.to_string())?;
    let out_root = PathBuf::from(format!("{}_rs", a.root));

    let mut cache = cache::Cache::new(&root);
    if let Some(cached) = cache.get(&a.file, &src) {
        let m = check::mod_name(&a.file);
        let out = out_root.join("src").join(format!("{m}.rs"));
        return Ok(Outcome::Done {
            rust: cached.rust_code.clone(),
            out_path: out.display().to_string(),
            checked: cached.checked,
            errors: String::new()
        });
    }

    let mut deps_ctx = String::new();
    for d in &a.deps {
        let dm = check::mod_name(d);
        if let Ok(code) = fs::read_to_string(out_root.join("src").join(format!("{dm}.rs"))) {
            deps_ctx.push_str(&format!("\n\nMigrated `{d}` as crate::{dm}:\n{}", graph::signatures(&code)));
        }
    }
    let mut crates_ctx = String::new();
    if !a.crates.is_empty() {
        crates_ctx.push_str("\n\nRust crates for Python libs (in Cargo.toml):\n");
        for (py, rc) in &a.crates {
            crates_ctx.push_str(&format!("- {py} -> {rc}\n"));
        }
    }
    let mut docs_ctx = String::new();
    let nonempty_docs: Vec<(&String, &String)> = a.docs.iter().filter(|(_, d)| !d.trim().is_empty()).collect();
    if !nonempty_docs.is_empty() {
        docs_ctx.push_str("\n\nLibrary docs:\n");
        for (py, d) in nonempty_docs {
            docs_ctx.push_str(&format!("### {py}\n{d}\n\n"));
        }
    }
    let mut user = format!("File: {}\n\n{}{deps_ctx}{docs_ctx}{crates_ctx}", a.file, src);
    for (i, ans) in a.answers.iter().enumerate() {
        user.push_str(&format!("\n\nUser answer #{}: {}", i + 1, ans));
    }
    let reply = a.provider.chat(&system_prompt(&a.description, &a.lang), &user).await.map_err(|e| e.to_string())?;
    if let Some(question) = parse_ask(&reply) {
        return Ok(Outcome::Ask { question });
    }
    let sys = system_prompt(&a.description, &a.lang);
    let pkg = root.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| "project".into());
    let m = check::mod_name(&a.file);
    check::ensure_crate(&out_root, &pkg).map_err(|e| e.to_string())?;
    check::register_mod(&out_root, &m).map_err(|e| e.to_string())?;
    let out = out_root.join("src").join(format!("{m}.rs"));

    let mut rust = strip_fence(&reply);
    let mut errors = String::new();
    for attempt in 0..=MAX_FIX {
        fs::write(&out, &rust).map_err(|e| e.to_string())?;
        check::add_deps(&out_root, &rust).await;
        check::add_named_crates(&out_root, a.crates.values()).await;
        errors = check::check(&out_root, &m).await.map_err(|e| e.to_string())?;
        if errors.is_empty() || attempt == MAX_FIX {
            break;
        }
        let fix = format!(
            "Python source:\n{src}\n\nYour Rust:\n{rust}\n\n{deps_ctx}{docs_ctx}{crates_ctx}\n\nAttempt {}/{MAX_FIX}\ncargo check errors:\n{errors}\n\nReturn the full corrected Rust file only.",
            attempt + 1
        );
        rust = strip_fence(&a.provider.chat(&sys, &fix).await.map_err(|e| e.to_string())?);
    }

    cache.set(a.file.clone(), &src, rust.clone(), errors.is_empty());
    let _ = cache.save();

    Ok(Outcome::Done { rust, out_path: out.display().to_string(), checked: errors.is_empty(), errors })
}

const KEYRING_SERVICE: &str = "py2rs";

#[tauri::command]
fn save_key(provider: String, key: String) -> Result<(), String> {
    keyring::Entry::new(KEYRING_SERVICE, &provider)
        .and_then(|e| e.set_password(&key))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn load_key(provider: String) -> Option<String> {
    keyring::Entry::new(KEYRING_SERVICE, &provider).ok()?.get_password().ok()
}

#[derive(Deserialize, Serialize)]
struct FileResult {
    file: String,
    out_path: String,
    checked: bool,
    errors: String,
}

#[tauri::command]
async fn finalize(root: String, results: Vec<FileResult>) -> Result<(bool, String), String> {
    let out_root = PathBuf::from(format!("{root}_rs"));
    let (ok, errors) = check::check_all(&out_root).await.map_err(|e| e.to_string())?;

    let mut report = String::from("# Migration Report\n\n");
    let (clean, dirty): (Vec<_>, Vec<_>) = results.iter().partition(|r| r.checked);
    report.push_str(&format!("Clean: {}/{}\n\n", clean.len(), results.len()));
    if !dirty.is_empty() {
        report.push_str("## Need Manual Fix\n\n");
        for r in &dirty {
            report.push_str(&format!("### {} -> {}\n```\n{}\n```\n\n", r.file, r.out_path, r.errors));
        }
    }
    report.push_str("## Migrated Successfully\n\n");
    for r in &clean {
        report.push_str(&format!("- {} -> {}\n", r.file, r.out_path));
    }
    if !ok {
        report.push_str(&format!("\n## Full Project Build Errors\n```\n{errors}\n```\n"));
    }
    fs::write(out_root.join("REPORT.md"), &report).map_err(|e| e.to_string())?;
    Ok((ok, out_root.join("REPORT.md").display().to_string()))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            plan_project,
            detect_libs,
            migrate_file,
            save_key,
            load_key,
            finalize,
            is_first_run,
            save_first_run_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running app");
}

#[tauri::command]
fn is_first_run() -> bool {
    let config_path = dirs::config_dir()
        .map(|d| d.join("py2rs").join("first_run.marker"));

    match config_path {
        Some(path) => !path.exists(),
        None => true,
    }
}

#[tauri::command]
fn save_first_run_config(provider: Provider) -> Result<(), String> {
    let config_dir = dirs::config_dir()
        .ok_or("Cannot find config directory")?
        .join("py2rs");

    fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;

    let marker_path = config_dir.join("first_run.marker");
    fs::write(marker_path, "").map_err(|e| e.to_string())?;

    if let Some(key) = &provider.api_key {
        save_key(provider.kind.to_string(), key.clone())?;
    }

    Ok(())
}
