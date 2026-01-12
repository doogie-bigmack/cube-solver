//! Web Build Integration Tests
//!
//! Tests to verify the web build is properly configured and the bundle is valid.

use std::path::Path;

#[test]
fn test_wasm_bundle_exists() {
    let wasm_path = Path::new("web/pkg/rubiks-cube-solver_bg.wasm");
    assert!(
        wasm_path.exists(),
        "WASM bundle should exist at web/pkg/rubiks-cube-solver_bg.wasm"
    );
}

#[test]
fn test_javascript_bundle_exists() {
    let js_path = Path::new("web/pkg/rubiks-cube-solver.js");
    assert!(
        js_path.exists(),
        "JavaScript bundle should exist at web/pkg/rubiks-cube-solver.js"
    );
}

#[test]
fn test_index_html_exists() {
    let html_path = Path::new("web/index.html");
    assert!(
        html_path.exists(),
        "index.html should exist at web/index.html"
    );
}

#[test]
fn test_wasm_bundle_size() {
    let wasm_path = Path::new("web/pkg/rubiks-cube-solver_bg.wasm");
    if wasm_path.exists() {
        let metadata = std::fs::metadata(wasm_path).expect("Failed to read WASM file metadata");
        let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);
        println!("WASM bundle size: {:.2} MB", size_mb);

        // Requirement: Bundle size under 5MB
        assert!(
            size_mb < 5.0,
            "WASM bundle size ({:.2} MB) should be under 5 MB",
            size_mb
        );
    }
}

#[test]
fn test_total_bundle_size() {
    let wasm_path = Path::new("web/pkg/rubiks-cube-solver_bg.wasm");
    let js_path = Path::new("web/pkg/rubiks-cube-solver.js");

    if wasm_path.exists() && js_path.exists() {
        let wasm_size = std::fs::metadata(wasm_path)
            .expect("Failed to read WASM file")
            .len();
        let js_size = std::fs::metadata(js_path)
            .expect("Failed to read JS file")
            .len();

        let total_size_mb = (wasm_size + js_size) as f64 / (1024.0 * 1024.0);
        println!("Total bundle size: {:.2} MB", total_size_mb);

        // Requirement: Bundle size under 5MB total
        assert!(
            total_size_mb < 5.0,
            "Total bundle size ({:.2} MB) should be under 5 MB",
            total_size_mb
        );
    }
}

#[test]
fn test_index_html_loads_wasm() {
    let html_path = Path::new("web/index.html");
    if html_path.exists() {
        let content = std::fs::read_to_string(html_path).expect("Failed to read index.html");

        // Check that it imports the WASM module
        assert!(
            content.contains("./pkg/rubiks-cube-solver.js"),
            "index.html should import the WASM JavaScript bundle"
        );

        // Check that it has proper structure
        assert!(
            content.contains("<html"),
            "index.html should have proper HTML structure"
        );
        assert!(
            content.contains("</html>"),
            "index.html should have closing html tag"
        );
    }
}

#[test]
fn test_dioxus_config_exists() {
    let config_path = Path::new("Dioxus.toml");
    assert!(
        config_path.exists(),
        "Dioxus.toml config should exist"
    );
}

#[test]
fn test_dioxus_config_web_platform() {
    let config_path = Path::new("Dioxus.toml");
    if config_path.exists() {
        let content = std::fs::read_to_string(config_path).expect("Failed to read Dioxus.toml");

        // Check that web platform is configured
        assert!(
            content.contains("default_platform") || content.contains("web"),
            "Dioxus.toml should configure web platform"
        );
    }
}
