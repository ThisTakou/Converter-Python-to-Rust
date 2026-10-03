use std::{fs, path::Path, process::Stdio};
use tokio::process::Command;

pub fn mod_name(file: &str) -> String {
    let s: String = file
        .trim_end_matches(".py")
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect();
    let s = s.trim_matches('_').to_string();
    match s.chars().next() {
        None => "module".into(),
        Some(c) if c.is_ascii_digit() => format!("m_{s}"),
        _ => s,
    }
}

pub fn ensure_crate(out: &Path, pkg: &str) -> anyhow::Result<()> {
    fs::create_dir_all(out.join("src"))?;
    let toml = out.join("Cargo.toml");
    if !toml.exists() {
        let pkg = mod_name(pkg);
        fs::write(toml, format!("[package]\nname = \"{pkg}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n"))?;
    }
    let lib = out.join("src/lib.rs");
    if !lib.exists() {
        fs::write(lib, "")?;
    }
    Ok(())
}

pub fn register_mod(out: &Path, m: &str) -> anyhow::Result<()> {
    let lib = out.join("src/lib.rs");
    let cur = fs::read_to_string(&lib).unwrap_or_default();
    let line = format!("pub mod {m};");
    if !cur.lines().any(|l| l == line) {
        fs::write(lib, format!("{cur}{line}\n"))?;
    }
    Ok(())
}

pub async fn add_deps(out: &Path, code: &str) {
    for l in code.lines() {
        if let Some(rest) = l.trim().strip_prefix("// deps:") {
            for d in rest.split(',').map(str::trim) {
                let ok = !d.is_empty() && d.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
                if ok {
                    let _ = Command::new("cargo").args(["add", d]).current_dir(out).output().await;
                }
            }
        }
    }
}

pub async fn add_named_crates<'a>(out: &Path, crates: impl Iterator<Item = &'a String>) {
    for c in crates {
        let name = c.split_whitespace().next().unwrap_or("");
        let valid = !name.is_empty()
            && !name.contains("::")
            && !c.contains(" или ")
            && !c.contains(" or ")
            && name.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_');
        if valid {
            let _ = Command::new("cargo").args(["add", name]).current_dir(out).output().await;
        }
    }
}

async fn run_check(out: &Path) -> anyhow::Result<(bool, String)> {
    let o = Command::new("cargo")
        .args(["check", "--message-format=short", "--color=never"])
        .current_dir(out)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .await?;
    let text = String::from_utf8_lossy(&o.stderr).replace('\\', "/");
    Ok((o.status.success(), text))
}

pub async fn check(out: &Path, m: &str) -> anyhow::Result<String> {
    let (ok, text) = run_check(out).await?;
    if ok {
        return Ok(String::new());
    }
    let needle = format!("src/{m}.rs");
    let errors: Vec<&str> = text.lines().filter(|l| l.contains(&needle) && l.contains("error")).collect();

    let mut structured = String::new();
    for err in &errors {
        if err.contains("cannot find") {
            structured.push_str("[Missing import/definition]\n");
        } else if err.contains("mismatched types") {
            structured.push_str("[Type mismatch]\n");
        } else if err.contains("trait") {
            structured.push_str("[Trait requirement]\n");
        } else if err.contains("lifetime") {
            structured.push_str("[Lifetime issue]\n");
        }
        structured.push_str(err);
        structured.push('\n');
    }

    Ok(structured)
}

pub async fn check_all(out: &Path) -> anyhow::Result<(bool, String)> {
    let (ok, text) = run_check(out).await?;
    let errors: String = text.lines().filter(|l| l.contains("error")).collect::<Vec<_>>().join("\n");
    Ok((ok, errors))
}
