#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_map_contains_common_libs() {
        let catalog = crate_map::catalog();

        // Test some common Python libraries
        assert!(catalog.contains_key("requests"));
        assert!(catalog.contains_key("numpy"));
        assert!(catalog.contains_key("pandas"));
        assert!(catalog.contains_key("flask"));
        assert!(catalog.contains_key("django"));

        // Check that suggestions are not empty
        let requests_info = catalog.get("requests").unwrap();
        assert_eq!(requests_info.crate_hint, "reqwest");
        assert!(!requests_info.doc.is_empty());
    }

    #[test]
    fn test_crate_map_size() {
        let catalog = crate_map::catalog();

        // Should have 300+ entries after expansion
        assert!(
            catalog.len() >= 300,
            "Expected at least 300 library mappings, found {}",
            catalog.len()
        );
    }

    #[test]
    fn test_detect_libs_empty_project() {
        use std::env;
        use std::fs;

        let temp_dir = env::temp_dir().join("py2rs_test_empty");
        fs::create_dir_all(&temp_dir).ok();

        let result = detect_libs(temp_dir.to_string_lossy().to_string());

        // Empty project should return empty list
        assert_eq!(result.len(), 0);

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_lib_suggestion_serialization() {
        let suggestion = LibSuggestion {
            py: "requests".to_string(),
            suggestion: Some("reqwest".to_string()),
            doc: Some("HTTP client library".to_string()),
        };

        // Should serialize to JSON without error
        let json = serde_json::to_string(&suggestion).unwrap();
        assert!(json.contains("requests"));
        assert!(json.contains("reqwest"));
    }

    #[test]
    fn test_file_entry_path() {
        let entry = FileEntry {
            file: "test.py".to_string(),
            path: "/path/to/test.py".to_string(),
            relative: "test.py".to_string(),
        };

        assert_eq!(entry.file, "test.py");
        assert_eq!(entry.relative, "test.py");
    }
}

#[cfg(test)]
mod provider_tests {
    use super::*;

    #[test]
    fn test_provider_from_kind() {
        let openai = Provider::from_kind("openai");
        assert!(matches!(openai, Provider::OpenAI));

        let anthropic = Provider::from_kind("anthropic");
        assert!(matches!(anthropic, Provider::Anthropic));

        let llamacpp = Provider::from_kind("llamacpp");
        assert!(matches!(llamacpp, Provider::LlamaCpp));

        let unknown = Provider::from_kind("unknown");
        assert!(matches!(unknown, Provider::OpenAI)); // defaults to OpenAI
    }

    #[test]
    fn test_provider_serialization() {
        let provider = Provider::OpenAI;
        let json = serde_json::to_string(&provider).unwrap();
        assert!(json.contains("OpenAI") || json.contains("openai"));
    }
}
