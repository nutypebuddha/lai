//! Integration tests for the Bankai engine

use std::collections::HashMap;

use athena::bankai::Bankai;
use athena::formula::{Formula, FormulaRegistry};
use athena::shikai::{Intent, ShikaiQuery};
use athena::wheel::Domain;
use athena::zanpakuto::{AccessTier, Identity};

fn setup() -> (Bankai, Identity) {
    let mut registry = FormulaRegistry::new();
    registry
        .register_all(vec![
            Formula::atomic(
                "newtons_second",
                Domain::Shukra,
                vec!["mass", "acceleration"],
                "force",
                "mass * acceleration",
                "F = ma",
            ),
            Formula::atomic(
                "pythagorean",
                Domain::Mangala,
                vec!["a", "b"],
                "c",
                "sqrt(a^2 + b^2)",
                "Pythagorean theorem",
            ),
            Formula::atomic(
                "kinetic_energy",
                Domain::Shukra,
                vec!["mass", "velocity"],
                "ke",
                "0.5 * mass * velocity^2",
                "KE = \u{bd}mv\u{00b2}",
            ),
        ])
        .unwrap();
    let bankai = Bankai::new(registry);
    let identity = Identity {
        name: "test".to_string(),
        tier: AccessTier::Bankai,
        capabilities: vec![],
        scope: vec![],
        session: "test".to_string(),
    };
    (bankai, identity)
}

#[test]
fn test_bankai_evaluate_newtons_second() {
    let (bankai, _) = setup();
    let mut args = HashMap::new();
    args.insert("mass".to_string(), 5.0);
    args.insert("acceleration".to_string(), 9.8);
    let result = bankai.evaluate("newtons_second", &args).unwrap();
    assert!((result - 49.0).abs() < 0.001);
}

#[test]
fn test_bankai_evaluate_pythagorean() {
    let (bankai, _) = setup();
    let mut args = HashMap::new();
    args.insert("a".to_string(), 3.0);
    args.insert("b".to_string(), 4.0);
    let result = bankai.evaluate("pythagorean", &args).unwrap();
    assert!((result - 5.0).abs() < 0.001);
}

#[test]
fn test_bankai_chain_simple() {
    let (bankai, _) = setup();
    let mut args = HashMap::new();
    args.insert("mass".to_string(), 10.0);
    args.insert("acceleration".to_string(), 2.0);
    let result = bankai.chain(&["newtons_second"], &args).unwrap();
    assert!(result.success);
    assert!((result.final_output().unwrap() - 20.0).abs() < 0.001);
}

#[test]
fn test_bankai_chain_multi_step() {
    let (bankai, _) = setup();
    let mut args = HashMap::new();
    args.insert("mass".to_string(), 2.0);
    args.insert("acceleration".to_string(), 9.8);
    args.insert("velocity".to_string(), 5.0);
    let result = bankai
        .chain(&["newtons_second", "kinetic_energy"], &args)
        .unwrap();
    assert!(result.success);
    assert_eq!(result.steps.len(), 2);
}

#[test]
fn test_bankai_chain_threads_outputs() {
    let mut registry = FormulaRegistry::new();
    registry
        .register_all(vec![
            Formula::atomic(
                "step_a",
                Domain::Shukra,
                vec!["mass", "acceleration"],
                "force",
                "mass * acceleration",
                "F = ma",
            ),
            Formula::atomic(
                "step_b",
                Domain::Shukra,
                vec!["force", "distance"],
                "work",
                "force * distance",
                "W = F*d",
            ),
        ])
        .unwrap();
    let bankai = Bankai::new(registry);
    let mut args = HashMap::new();
    args.insert("mass".to_string(), 10.0);
    args.insert("acceleration".to_string(), 2.0);
    args.insert("distance".to_string(), 5.0);
    let result = bankai.chain(&["step_a", "step_b"], &args).unwrap();
    assert!(result.success, "chain should succeed with threaded outputs");
    assert_eq!(result.steps.len(), 2);
    // step_a: force = 10 * 2 = 20
    // step_b: work = 20 * 5 = 100
    assert!((result.final_output().unwrap() - 100.0).abs() < 0.001);
}

#[test]
fn test_bankai_chain_fails_on_missing_threaded_arg() {
    let mut registry = FormulaRegistry::new();
    registry
        .register_all(vec![
            Formula::atomic("step_a", Domain::Shukra, vec!["x"], "y", "x * 2", "double"),
            Formula::atomic(
                "step_b",
                Domain::Shukra,
                vec!["y"],
                "z",
                "y + 1",
                "increment",
            ),
        ])
        .unwrap();
    let bankai = Bankai::new(registry);
    let mut args = HashMap::new();
    args.insert("x".to_string(), 5.0);
    let result = bankai.chain(&["step_a", "step_b"], &args).unwrap();
    assert!(
        result.success,
        "step_a output `y` should be threaded into step_b input `y`"
    );
    assert_eq!(result.steps.len(), 2);
    assert!((result.final_output().unwrap() - 11.0).abs() < 0.001);
    // y = x * 2 = 10, then z = y + 1 = 11
}

#[test]
fn test_bankai_solve_evaluate_intent() {
    let (mut bankai, identity) = setup();
    let query = ShikaiQuery {
        original: "newtons_second mass=5 acceleration=9.8".to_string(),
        intent: Intent::Evaluate,
        domains: vec![Domain::Shukra],
        formula_ids: vec!["newtons_second".to_string()],
        args: vec![("mass".to_string(), 5.0), ("acceleration".to_string(), 9.8)],
        entity_context: None,
        level: None,
        cycle: None,
        ambiguity: vec![],
    };
    let solve = bankai.solve(&query, &identity);
    assert!(solve.success);
    assert!((solve.solved_value.unwrap() - 49.0).abs() < 0.1);
}

#[test]
fn test_bankai_solve_info() {
    let (mut bankai, identity) = setup();
    let query = ShikaiQuery {
        original: "info".to_string(),
        intent: Intent::Info,
        domains: vec![],
        formula_ids: vec![],
        args: vec![],
        entity_context: None,
        level: None,
        cycle: None,
        ambiguity: vec![],
    };
    let solve = bankai.solve(&query, &identity);
    assert!(solve.success);
    assert!(solve.summary.contains("Bankai released"));
}

#[test]
fn test_bankai_traverse() {
    let (bankai, _) = setup();
    let traversal = bankai.traverse(Domain::Mangala, 3);
    assert!(!traversal.path.is_empty());
    assert!(traversal.formula_count() > 0);
    assert_eq!(traversal.start, Domain::Mangala);
}

#[test]
fn test_bankai_compose() {
    let (bankai, _) = setup();
    let comp = bankai.compose(&["newtons_second", "pythagorean"]).unwrap();
    assert_eq!(comp.len(), 2);
    assert!(!comp.is_empty());
    // Domains: [Taurus, Aries] — formula domains with window + trailing
    assert_eq!(comp.domains_traversed.len(), 2);
}

#[test]
fn test_bankai_compose_not_found() {
    let (bankai, _) = setup();
    let result = bankai.compose(&["nonexistent_formula"]);
    assert!(result.is_err());
}

#[test]
fn test_bankai_find_path() {
    let (bankai, _) = setup();
    let have = vec!["mass".to_string(), "acceleration".to_string()];
    let path = bankai.find_path(&have, "force", 5).unwrap();
    assert_eq!(path, vec!["newtons_second"]);
}

#[test]
fn test_bankai_find_path_not_found() {
    let (bankai, _) = setup();
    let have = vec!["magic".to_string()];
    let err = bankai.find_path(&have, "unicorn", 5).unwrap_err();
    assert!(matches!(
        err,
        athena::bankai::BankaiError::PathNotFound { .. }
    ));
}
