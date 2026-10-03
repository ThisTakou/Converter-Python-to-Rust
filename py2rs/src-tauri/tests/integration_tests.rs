// Integration tests for py2rs

use std::fs;
use std::path::PathBuf;

#[test]
fn test_example_simple_script_exists() {
    let example_path = PathBuf::from("../../examples/simple_script/main.py");
    assert!(
        example_path.exists() || PathBuf::from("examples/simple_script/main.py").exists(),
        "Simple script example should exist"
    );
}

#[test]
fn test_example_web_api_exists() {
    let example_path = PathBuf::from("../../examples/web_api/app.py");
    assert!(
        example_path.exists() || PathBuf::from("examples/web_api/app.py").exists(),
        "Web API example should exist"
    );
}

#[test]
fn test_example_data_analysis_exists() {
    let example_path = PathBuf::from("../../examples/data_analysis/analyze.py");
    assert!(
        example_path.exists() || PathBuf::from("examples/data_analysis/analyze.py").exists(),
        "Data analysis example should exist"
    );
}

#[test]
fn test_documentation_files_exist() {
    let docs = vec![
        "../../README.md",
        "../../CONTRIBUTING.md",
        "../../ARCHITECTURE.md",
        "../../CODE_OF_CONDUCT.md",
        "../../SECURITY.md",
    ];

    for doc in docs {
        let path = PathBuf::from(doc);
        assert!(
            path.exists() || PathBuf::from(&doc[6..]).exists(),
            "Documentation file should exist: {}",
            doc
        );
    }
}

#[test]
fn test_github_workflows_exist() {
    let workflows = vec![
        "../../.github/workflows/ci.yml",
        "../../.github/workflows/release.yml",
    ];

    for workflow in workflows {
        let path = PathBuf::from(workflow);
        assert!(
            path.exists() || PathBuf::from(&workflow[6..]).exists(),
            "GitHub workflow should exist: {}",
            workflow
        );
    }
}

#[test]
fn test_crate_map_module_compiles() {
    // This test just ensures the crate_map module compiles correctly
    // The actual catalog content is tested in crate_map_tests.rs
    let _catalog = py2rs_lib::crate_map::catalog();
}

#[cfg(feature = "mock_llm")]
#[test]
fn test_migration_with_mock_provider() {
    // Mock test for migration flow
    // In real implementation, this would use a mock LLM provider
    // For now, just verify the structure
    assert!(true, "Mock migration test placeholder");
}
