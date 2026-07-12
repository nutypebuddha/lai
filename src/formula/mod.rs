/// Pure function: Validate a formula ID string.
pub fn validate_formula_id(formula_id: &str) -> bool {
    !formula_id.is_empty() && formula_id.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// Pure function: Extract domain from a formula ID.
pub fn extract_formula_domain(formula_id: &str) -> &str {
    formula_id.split('_').next().unwrap_or("")
}

/// Pure function: Check if formula ID is atomic type.
pub fn is_atomic_formula(formula_id: &str) -> bool {
    extract_formula_domain(formula_id) == "atomic"
}

/// Pure function: Check if formula ID is bridging type.
pub fn is_bridging_formula(formula_id: &str) -> bool {
    extract_formula_domain(formula_id) == "bridging"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_formula_id_basic() {
        assert!(validate_formula_id("atomic_graha"));
        assert!(validate_formula_id("bridging_001"));
        assert!(!validate_formula_id(""));
        assert!(!validate_formula_id("has space"));
    }

    #[test]
    fn extract_formula_domain_basic() {
        assert_eq!(extract_formula_domain("atomic_graha"), "atomic");
        assert_eq!(extract_formula_domain("bridging_001"), "bridging");
    }

    #[test]
    fn is_atomic_formula_basic() {
        assert!(is_atomic_formula("atomic_graha"));
        assert!(!is_atomic_formula("bridging_001"));
    }

    #[test]
    fn is_bridging_formula_basic() {
        assert!(is_bridging_formula("bridging_001"));
        assert!(!is_bridging_formula("atomic_graha"));
    }
}
