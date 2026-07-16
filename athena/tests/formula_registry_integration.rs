//! Integration tests for the Formula Registry
//!
//! Tests the real TOML-loaded formula registry, not just mock formulas.
//! These verify that the actual formula files load and are searchable.

use athena::formula::{Formula, FormulaRegistry};
use athena::wheel::Domain;

/// Load all formula TOML files from the formulas/ directory tree.
fn load_real_registry() -> FormulaRegistry {
    let mut registry = FormulaRegistry::new();
    let formula_dirs = ["formulas/atomic", "formulas/bridging", "formulas/nonmath"];
    for dir in &formula_dirs {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "toml") {
                    let _ = registry.load_from_file(&path);
                }
            }
        }
    }
    registry
}

fn create_test_registry() -> FormulaRegistry {
    let mut registry = FormulaRegistry::new();
    registry
        .register_all(vec![
            Formula::atomic(
                "test_add",
                Domain::Mangala,
                vec!["a", "b"],
                "sum",
                "a + b",
                "Addition",
            ),
            Formula::atomic(
                "test_mul",
                Domain::Shukra,
                vec!["x", "y"],
                "product",
                "x * y",
                "Multiplication",
            ),
        ])
        .unwrap();
    registry
}

#[test]
fn test_registry_len() {
    let registry = create_test_registry();
    assert_eq!(registry.len(), 2);
}

#[test]
fn test_registry_search_by_id() {
    let registry = create_test_registry();
    let results = registry.search("test_add");
    assert!(!results.is_empty());
    assert!(results.iter().any(|f| f.id == "test_add"));
}

#[test]
fn test_registry_search_by_description() {
    let registry = create_test_registry();
    let results = registry.search("Addition");
    assert_eq!(results.len(), 1);
}

#[test]
fn test_registry_by_domain() {
    let registry = create_test_registry();
    let aries = registry.by_domain(Domain::Mangala);
    assert_eq!(aries.len(), 1);
}

#[test]
fn test_registry_get_nonexistent() {
    let registry = create_test_registry();
    assert!(registry.get("nonexistent_formula").is_none());
}

#[test]
fn test_registry_search_semantic() {
    let registry = create_test_registry();
    let results = registry.search_semantic("add", 5);
    assert!(results.iter().any(|f| f.id == "test_add"));
}

#[test]
fn test_real_registry_loads_all_formulas() {
    let registry = load_real_registry();
    assert!(
        !registry.is_empty(),
        "Real registry should have formulas loaded from TOML"
    );
    assert!(
        !registry.by_domain(Domain::Mangala).is_empty(),
        "Aries should have formulas"
    );
    assert!(
        !registry.by_domain(Domain::Shukra).is_empty(),
        "Shukra (Taurus/Venus — Physics) should have formulas"
    );
    assert!(
        !registry.by_domain(Domain::Mangala).is_empty(),
        "Scorpio should have formulas"
    );
    assert!(registry.get("add").is_some(), "Should have 'add' formula");
    assert!(
        registry.get("mass_energy_equivalence").is_some(),
        "Should have 'mass_energy_equivalence' formula"
    );
    assert!(
        registry.get("entropy_change").is_some(),
        "Should have 'entropy_change' formula"
    );
}

#[test]
fn test_registry_search_multiple_results() {
    let registry = load_real_registry();
    let results = registry.search("energy");
    assert!(
        results.len() >= 2,
        "Should find multiple formulas containing 'energy', got {}",
        results.len()
    );
}

#[test]
fn test_registry_search_tags() {
    let registry = load_real_registry();
    let results = registry.search("arithmetic");
    assert!(
        !results.is_empty(),
        "Should find formulas tagged 'arithmetic'"
    );
}

#[test]
fn test_registry_all_returns_sorted() {
    let registry = load_real_registry();
    let all = registry.all();
    assert!(!all.is_empty(), "Registry should have at least one formula");
    for i in 1..all.len() {
        assert!(
            all[i - 1].id <= all[i].id,
            "Formulas should be sorted by ID"
        );
    }
}
