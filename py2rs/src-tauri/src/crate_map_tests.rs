#[cfg(test)]
mod crate_map_tests {
    use super::*;

    #[test]
    fn test_catalog_has_required_size() {
        let catalog = catalog();
        assert!(
            catalog.len() >= 300,
            "Catalog should have at least 300 entries, found {}",
            catalog.len()
        );
    }

    #[test]
    fn test_common_libraries_exist() {
        let catalog = catalog();

        // Web frameworks
        assert!(catalog.contains_key("flask"));
        assert!(catalog.contains_key("django"));
        assert!(catalog.contains_key("fastapi"));

        // HTTP clients
        assert!(catalog.contains_key("requests"));
        assert!(catalog.contains_key("httpx"));

        // Data science
        assert!(catalog.contains_key("numpy"));
        assert!(catalog.contains_key("pandas"));
        assert!(catalog.contains_key("matplotlib"));

        // Databases
        assert!(catalog.contains_key("sqlalchemy"));
        assert!(catalog.contains_key("psycopg2"));
        assert!(catalog.contains_key("pymongo"));

        // Testing
        assert!(catalog.contains_key("pytest"));
        assert!(catalog.contains_key("unittest"));

        // CLI
        assert!(catalog.contains_key("argparse"));
        assert!(catalog.contains_key("click"));
    }

    #[test]
    fn test_library_info_not_empty() {
        let catalog = catalog();

        for (py_lib, info) in catalog.iter() {
            assert!(
                !info.crate_hint.is_empty(),
                "crate_hint should not be empty for {}",
                py_lib
            );
            assert!(
                !info.doc.is_empty(),
                "doc should not be empty for {}",
                py_lib
            );
        }
    }

    #[test]
    fn test_specific_mappings() {
        let catalog = catalog();

        // Test specific known mappings
        assert_eq!(catalog.get("requests").unwrap().crate_hint, "reqwest");
        assert_eq!(catalog.get("numpy").unwrap().crate_hint, "ndarray");
        assert_eq!(catalog.get("pandas").unwrap().crate_hint, "polars");
        assert_eq!(catalog.get("flask").unwrap().crate_hint, "axum");
    }

    #[test]
    fn test_documentation_quality() {
        let catalog = catalog();

        // Check that documentation contains useful keywords
        let requests_doc = catalog.get("requests").unwrap().doc;
        assert!(
            requests_doc.contains("HTTP") || requests_doc.contains("client"),
            "Documentation should mention HTTP or client"
        );

        let numpy_doc = catalog.get("numpy").unwrap().doc;
        assert!(
            numpy_doc.contains("array") || numpy_doc.contains("ndarray"),
            "Documentation should mention arrays"
        );
    }

    #[test]
    fn test_stdlib_no_suggestion_list() {
        let stdlib = stdlib_no_suggestion();

        // Should contain common stdlib modules
        assert!(stdlib.contains(&"sys"));
        assert!(stdlib.contains(&"typing"));
        assert!(stdlib.contains(&"abc"));
        assert!(stdlib.contains(&"functools"));
    }

    #[test]
    fn test_no_duplicate_keys() {
        let catalog = catalog();
        let keys: Vec<_> = catalog.keys().collect();

        for (i, key1) in keys.iter().enumerate() {
            for key2 in keys.iter().skip(i + 1) {
                assert_ne!(
                    key1, key2,
                    "Found duplicate key in catalog: {}",
                    key1
                );
            }
        }
    }

    #[test]
    fn test_async_libraries_mapped_correctly() {
        let catalog = catalog();

        // Async libraries should map to Rust async equivalents
        let asyncio = catalog.get("asyncio").unwrap();
        assert!(asyncio.crate_hint.contains("tokio"));

        let aiohttp = catalog.get("aiohttp").unwrap();
        assert!(aiohttp.doc.contains("async") || aiohttp.doc.contains("tokio"));
    }

    #[test]
    fn test_web_frameworks_coverage() {
        let catalog = catalog();

        let web_frameworks = vec![
            "flask", "django", "fastapi", "starlette",
            "tornado", "sanic", "bottle", "cherrypy", "pyramid"
        ];

        for framework in web_frameworks {
            assert!(
                catalog.contains_key(framework),
                "Missing web framework: {}",
                framework
            );
        }
    }

    #[test]
    fn test_data_science_coverage() {
        let catalog = catalog();

        let ds_libs = vec![
            "numpy", "pandas", "scipy", "matplotlib",
            "sklearn", "tensorflow", "torch", "seaborn", "plotly"
        ];

        for lib in ds_libs {
            assert!(
                catalog.contains_key(lib),
                "Missing data science library: {}",
                lib
            );
        }
    }

    #[test]
    fn test_database_coverage() {
        let catalog = catalog();

        let db_libs = vec![
            "sqlalchemy", "psycopg2", "pymongo", "redis",
            "pymysql", "sqlite3", "cassandra-driver", "elasticsearch"
        ];

        for lib in db_libs {
            assert!(
                catalog.contains_key(lib),
                "Missing database library: {}",
                lib
            );
        }
    }
}
