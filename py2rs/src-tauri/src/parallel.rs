use crate::graph::PlanItem;
use std::collections::HashSet;

pub fn group_by_level(plan: &[PlanItem]) -> Vec<Vec<String>> {
    let mut levels = Vec::new();
    let mut done = HashSet::new();

    loop {
        let ready: Vec<String> = plan
            .iter()
            .filter(|p| !done.contains(&p.file))
            .filter(|p| p.deps.iter().all(|d| done.contains(d)))
            .map(|p| p.file.clone())
            .collect();

        if ready.is_empty() {
            break;
        }

        for f in &ready {
            done.insert(f.clone());
        }
        levels.push(ready);
    }

    levels
}
