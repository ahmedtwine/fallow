use super::common::{create_config, fixture_path};

#[test]
fn three_level_star_chain_used_exports_propagate() {
    let root = fixture_path("re-export-chains");
    let config = create_config(root);
    let results = fallow_core::analyze(&config).expect("analysis should succeed");

    let unused_export_names: Vec<&str> = results
        .unused_exports
        .iter()
        .map(|e| e.export.export_name.as_str())
        .collect();

    assert!(
        !unused_export_names.contains(&"alpha"),
        "alpha should propagate through 3-level star chain, found: {unused_export_names:?}"
    );
    assert!(
        !unused_export_names.contains(&"beta"),
        "beta should propagate through 3-level star chain, found: {unused_export_names:?}"
    );
}

#[test]
fn three_level_star_chain_unused_exports_detected() {
    let root = fixture_path("re-export-chains");
    let config = create_config(root);
    let results = fallow_core::analyze(&config).expect("analysis should succeed");

    let unused_export_names: Vec<&str> = results
        .unused_exports
        .iter()
        .map(|e| e.export.export_name.as_str())
        .collect();

    assert!(
        unused_export_names.contains(&"gamma"),
        "gamma should be unused (not imported), found: {unused_export_names:?}"
    );
    assert!(
        unused_export_names.contains(&"delta"),
        "delta should be unused (not imported), found: {unused_export_names:?}"
    );
}

#[test]
fn three_level_star_chain_no_unused_files() {
    let root = fixture_path("re-export-chains");
    let config = create_config(root);
    let results = fallow_core::analyze(&config).expect("analysis should succeed");

    assert!(
        results.unused_files.is_empty(),
        "no files should be unused in re-export chain fixture, found: {:?}",
        results
            .unused_files
            .iter()
            .map(|f| &f.file.path)
            .collect::<Vec<_>>()
    );
}
