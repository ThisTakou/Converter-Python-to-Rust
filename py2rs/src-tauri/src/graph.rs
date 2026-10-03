use serde::Serialize;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::{fs, path::Path};
use walkdir::WalkDir;

const SKIP: [&str; 6] = ["venv", ".venv", "__pycache__", ".git", "node_modules", "site-packages"];

#[derive(Serialize, Clone)]
pub struct PlanItem {
    pub file: String,
    pub deps: Vec<String>,
}

struct Imp {
    dots: usize,
    module: String,
    names: Vec<String>,
}

fn key(p: &Path) -> String {
    p.display().to_string().replace('\\', "/")
}

pub fn list_py(root: &Path) -> Vec<String> {
    let mut v: Vec<String> = WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !SKIP.iter().any(|s| e.file_name() == *s))
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|x| x == "py"))
        .filter_map(|e| e.path().strip_prefix(root).ok().map(key))
        .collect();
    v.sort();
    v
}

fn parse_imports(src: &str) -> Vec<Imp> {
    let mut out = vec![];
    for line in src.lines() {
        let l = line.trim();
        if let Some(rest) = l.strip_prefix("import ") {
            for part in rest.split(',') {
                if let Some(name) = part.split_whitespace().next() {
                    out.push(Imp { dots: 0, module: name.to_string(), names: vec![] });
                }
            }
        } else if let Some(rest) = l.strip_prefix("from ") {
            let Some((m, n)) = rest.split_once(" import ") else { continue };
            let m = m.trim();
            let dots = m.chars().take_while(|c| *c == '.').count();
            let names = n
                .trim()
                .trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .filter_map(|x| x.split_whitespace().next())
                .filter(|x| *x != "*")
                .map(String::from)
                .collect();
            out.push(Imp { dots, module: m[dots..].to_string(), names });
        }
    }
    out
}

fn resolve(files: &HashSet<String>, base: &str, module: &str) -> Option<String> {
    let rel = module.replace('.', "/");
    let path = if base.is_empty() { rel } else { format!("{base}/{rel}") };
    [format!("{path}.py"), format!("{path}/__init__.py")].into_iter().find(|c| files.contains(c))
}

fn first_prefix(files: &HashSet<String>, base: &str, module: &str) -> Option<String> {
    let mut parts: Vec<&str> = module.split('.').collect();
    while !parts.is_empty() {
        if let Some(x) = resolve(files, base, &parts.join(".")) {
            return Some(x);
        }
        parts.pop();
    }
    None
}

fn deps_of(f: &str, src: &str, files: &HashSet<String>) -> Vec<String> {
    let pkg = f.rsplit_once('/').map(|(d, _)| d).unwrap_or("");
    let mut deps = BTreeSet::new();
    for imp in parse_imports(src) {
        let mut found = vec![];
        if imp.dots == 0 {
            for base in ["", pkg] {
                found.extend(first_prefix(files, base, &imp.module));
                for n in &imp.names {
                    found.extend(resolve(files, base, &format!("{}.{}", imp.module, n)));
                }
                if !found.is_empty() {
                    break;
                }
            }
        } else {
            let mut base = pkg.to_string();
            for _ in 1..imp.dots {
                base = base.rsplit_once('/').map(|(d, _)| d).unwrap_or("").to_string();
            }
            if !imp.module.is_empty() {
                found.extend(resolve(files, &base, &imp.module));
            }
            for n in &imp.names {
                let m = if imp.module.is_empty() { n.clone() } else { format!("{}.{}", imp.module, n) };
                found.extend(resolve(files, &base, &m));
            }
        }
        deps.extend(found);
    }
    deps.into_iter().filter(|d| d != f).collect()
}

fn visit(f: &String, deps: &HashMap<String, Vec<String>>, seen: &mut HashSet<String>, order: &mut Vec<String>) {
    if !seen.insert(f.clone()) {
        return;
    }
    for d in &deps[f] {
        visit(d, deps, seen, order);
    }
    order.push(f.clone());
}

pub fn plan(root: &Path) -> Vec<PlanItem> {
    let list = list_py(root);
    let set: HashSet<String> = list.iter().cloned().collect();
    let deps: HashMap<String, Vec<String>> = list
        .iter()
        .map(|f| (f.clone(), deps_of(f, &fs::read_to_string(root.join(f)).unwrap_or_default(), &set)))
        .collect();
    let (mut seen, mut order) = (HashSet::new(), vec![]);
    for f in &list {
        visit(f, &deps, &mut seen, &mut order);
    }
    order.into_iter().map(|f| PlanItem { deps: deps[&f].clone(), file: f }).collect()
}

pub fn external_modules(root: &Path) -> Vec<String> {
    let list = list_py(root);
    let set: HashSet<String> = list.iter().cloned().collect();
    let skip = crate::crate_map::stdlib_no_suggestion();
    let mut found = BTreeSet::new();
    for f in &list {
        let src = fs::read_to_string(root.join(f)).unwrap_or_default();
        for imp in parse_imports(&src) {
            if imp.dots > 0 {
                continue;
            }
            let top = imp.module.split('.').next().unwrap_or(&imp.module).to_string();
            if resolve(&set, "", &top).is_none() && !skip.contains(&top.as_str()) {
                found.insert(top);
            }
        }
    }
    found.into_iter().collect()
}

pub fn signatures(rust: &str) -> String {
    rust.lines()
        .map(str::trim)
        .filter(|l| l.starts_with("pub ") && !l.starts_with("pub mod") && !l.starts_with("pub use"))
        .map(|l| l.split('{').next().unwrap_or(l).trim_end_matches(';').trim().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
