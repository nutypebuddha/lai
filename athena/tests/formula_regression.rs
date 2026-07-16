use std::collections::HashMap;

use athena::bankai::Bankai;
use athena::formula::FormulaRegistry;

fn load_all_formulas() -> FormulaRegistry {
    let mut registry = FormulaRegistry::new();
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("formulas");

    // Load all atomic domain files
    for entry in std::fs::read_dir(base.join("atomic")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "toml") {
            registry.load_from_file(&path).unwrap();
        }
    }

    // Load bridging formulas
    for entry in std::fs::read_dir(base.join("bridging")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "toml") {
            registry.load_from_file(&path).unwrap();
        }
    }

    // Load nonmath formulas
    let nonmath_dir = base.join("nonmath");
    if nonmath_dir.exists() {
        for entry in std::fs::read_dir(&nonmath_dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "toml") {
                registry.load_from_file(&path).unwrap();
            }
        }
    }

    registry
}

#[test]
fn test_shannon_entropy_regression() {
    let registry = load_all_formulas();
    let bankai = Bankai::new(registry);

    // shannon_entropy: H = -p * log2(p)
    let mut args = HashMap::new();
    args.insert("probability".to_string(), 0.5);
    let result = bankai.evaluate("shannon_entropy", &args).unwrap();
    assert!(
        (result - 0.5).abs() < 1e-10,
        "shannon_entropy(0.5) should be 0.5, got {}",
        result
    );

    args.insert("probability".to_string(), 1.0);
    let result = bankai.evaluate("shannon_entropy", &args).unwrap();
    assert!(
        (result - 0.0).abs() < 1e-10,
        "shannon_entropy(1.0) should be 0.0, got {}",
        result
    );

    args.insert("probability".to_string(), 0.25);
    let result = bankai.evaluate("shannon_entropy", &args).unwrap();
    let expected = -0.25 * (0.25f64.log(10.0) / 2.0f64.log(10.0));
    assert!(
        (result - expected).abs() < 1e-10,
        "shannon_entropy(0.25) mismatch"
    );
}

#[test]
fn test_known_formula_outputs_dont_regress() {
    let registry = load_all_formulas();
    let bankai = Bankai::new(registry);

    // add: sum = a + b
    let mut args = HashMap::new();
    args.insert("a".to_string(), 3.0);
    args.insert("b".to_string(), 4.0);
    let result = bankai.evaluate("add", &args).unwrap();
    assert!((result - 7.0).abs() < 1e-10);

    // multiply: product = a * b
    let mut args = HashMap::new();
    args.insert("a".to_string(), 6.0);
    args.insert("b".to_string(), 7.0);
    let result = bankai.evaluate("multiply", &args).unwrap();
    assert!((result - 42.0).abs() < 1e-10);

    // mass_energy_equivalence: E = m * c^2
    let mut args = HashMap::new();
    args.insert("mass".to_string(), 1e-6);
    args.insert("c".to_string(), 299792458.0);
    let expected = 1e-6 * 299792458.0 * 299792458.0;
    let result = bankai.evaluate("mass_energy_equivalence", &args).unwrap();
    assert!((result - expected).abs() < 1.0);
}
