use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn parse_requirements(root: &Path) -> HashMap<String, String> {
    let mut deps = HashMap::new();

    if let Ok(content) = fs::read_to_string(root.join("requirements.txt")) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((name, version)) = parse_dep_line(line) {
                deps.insert(name, version);
            }
        }
    }

    if let Ok(content) = fs::read_to_string(root.join("pyproject.toml")) {
        if let Some(deps_section) = extract_pyproject_deps(&content) {
            for line in deps_section.lines() {
                if let Some((name, version)) = parse_dep_line(line.trim().trim_matches(|c| c == '"' || c == '\'')) {
                    deps.entry(name).or_insert(version);
                }
            }
        }
    }

    deps
}

fn parse_dep_line(line: &str) -> Option<(String, String)> {
    let mut parts = line.split(&['=', '>', '<', '~', '!'][..]);
    let name = parts.next()?.trim().to_lowercase();

    if name.is_empty() || name.starts_with('-') {
        return None;
    }

    let version = line.splitn(2, &['=', '>', '<', '~'][..])
        .nth(1)
        .map(|v| v.trim_matches(|c: char| !c.is_ascii_digit() && c != '.').to_string())
        .unwrap_or_default();

    Some((name, version))
}

fn extract_pyproject_deps(content: &str) -> Option<String> {
    let mut in_deps = false;
    let mut deps = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed == "[project.dependencies]" || trimmed == "dependencies = [" {
            in_deps = true;
            continue;
        }

        if in_deps {
            if trimmed.starts_with('[') && !trimmed.starts_with("[[") {
                break;
            }
            if trimmed == "]" {
                break;
            }
            deps.push_str(line);
            deps.push('\n');
        }
    }

    if deps.is_empty() {
        None
    } else {
        Some(deps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dep_line() {
        assert_eq!(parse_dep_line("requests==2.28.0"), Some(("requests".into(), "2.28.0".into())));
        assert_eq!(parse_dep_line("flask>=2.0.0"), Some(("flask".into(), "2.0.0".into())));
        assert_eq!(parse_dep_line("numpy"), Some(("numpy".into(), String::new())));
        assert_eq!(parse_dep_line("# comment"), None);
    }
}
