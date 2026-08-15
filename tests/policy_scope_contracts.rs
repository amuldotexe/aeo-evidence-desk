//! TEST-POLICY-012 / REQ-TAURI-003.0

use std::{fs, path::Path};

#[test]
fn test_req_tauri_003_omits_live_data_and_privileged_plugins() {
    let manifest_paths = ["Cargo.toml", "src-tauri/Cargo.toml", "ui/package.json"];
    let forbidden_dependency_markers = [
        "reqwest",
        "ureq",
        "openai",
        "anthropic",
        "google-generative",
        "sqlx",
        "rusqlite",
        "diesel",
        "tauri-plugin-http",
        "tauri-plugin-fs",
        "tauri-plugin-shell",
        "tauri-plugin-store",
        "tauri-plugin-updater",
        "axios",
    ];

    for manifest_path in manifest_paths {
        let manifest_text = fs::read_to_string(manifest_path).expect("manifest should exist");
        let normalized_manifest = manifest_text.to_lowercase();
        for forbidden_marker in forbidden_dependency_markers {
            assert!(
                !normalized_manifest.contains(forbidden_marker),
                "{manifest_path} must not declare `{forbidden_marker}`"
            );
        }
    }

    let capability_text = fs::read_to_string("src-tauri/capabilities/default.json")
        .expect("capability configuration should exist");
    let capability_value: serde_json::Value =
        serde_json::from_str(&capability_text).expect("capability configuration should be JSON");
    assert_eq!(
        capability_value["permissions"],
        serde_json::json!(["core:default"])
    );

    for production_source in [
        "src/fixture_data.rs",
        "src-tauri/src/commands.rs",
        "ui/src/api.ts",
        "ui/src/app.ts",
    ] {
        let source_text = fs::read_to_string(Path::new(production_source))
            .expect("production source should exist")
            .to_lowercase();
        for forbidden_marker in ["fetch(", "xmlhttprequest", "localstorage", "indexeddb"] {
            assert!(
                !source_text.contains(forbidden_marker),
                "{production_source} must not use `{forbidden_marker}`"
            );
        }
    }
}
