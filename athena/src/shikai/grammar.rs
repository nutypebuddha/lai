//! # Grammar domain inference for Shikai
//!
//! Provides the shared `word_stem_match` helper and a simplified
//! domain inference that now uses entity/domain keyword matching.
//! The old non-grammar rule system has been replaced by the
//! zodiac wheel and entity-based domain resolution.

use crate::formula::FormulaRegistry;
use crate::wheel::Domain;

/// Helper: does any word from the query have a stem match in the target text?
///
/// A "stem match" means:
/// - One word is a substring of the other (e.g. "serial" matches "serial_comma_rule")
/// - OR both words share a common prefix of at least 4 characters (e.g. "presents" and
///   "presentation" both share the stem "present")
///
/// To avoid false positives, the shorter word must be >= 4 characters before any
/// substring match is considered.
///
/// Both `query_word` and `target` should be pre-lowercased for best performance —
/// callers in the hot path already pre-lowercase their inputs.
#[allow(dead_code)]
pub(crate) fn word_stem_match(query_word: &str, target: &str) -> bool {
    // Direct substring match — minimum length guard to avoid false positives
    if query_word.len() >= 4 && target.contains(query_word) {
        return true;
    }
    if target.len() >= 4 && query_word.contains(target) {
        return true;
    }

    // Check each whitespace-separated word in target for stem match
    target.split_whitespace().any(|tw| {
        // Both words must be >= 4 chars to stem-match
        if tw.len() < 4 || query_word.len() < 4 {
            return false;
        }
        // Check if either is a substring of the other
        if tw.contains(query_word) || query_word.contains(tw) {
            return true;
        }
        // Check for common prefix of length >= 4 (stem match like "presents"/"presentation")
        let common_len = tw
            .as_bytes()
            .iter()
            .zip(query_word.as_bytes().iter())
            .take_while(|(a, b)| a == b)
            .count();
        common_len >= 4
    })
}

/// Infer a domain from keywords in the query.
///
/// Uses domain archetypes and zodiac names as keywords.
/// This is a simplified version that replaces the old grammar-rule-based inference.
pub(crate) fn infer_domain_from_grammar(
    _formulas: &FormulaRegistry,
    query: &str,
) -> Option<Domain> {
    let lower = query.to_lowercase();

    // Direct domain name mentions
    for d in crate::wheel::ALL_DOMAINS.iter() {
        let name = d.full_name_lower();
        if lower.contains(name) {
            return Some(*d);
        }
        // Check symbol
        if lower.contains(d.symbol()) {
            return Some(*d);
        }
        // Check knowledge domain keywords (pre-lowered — no allocation)
        if lower.contains(d.knowledge_domain_lower()) {
            return Some(*d);
        }
    }

    // Archetype keyword mapping
    let archetype_map: &[(&[&str], Domain)] = &[
        (
            &[
                "math", "logic", "number", "count", "compute", "prove", "theorem", "proof",
            ],
            Domain::Mangala,
        ),
        (
            &[
                "physics",
                "force",
                "mass",
                "energy",
                "motion",
                "chemistry",
                "atom",
                "molecule",
            ],
            Domain::Shukra,
        ),
        (
            &[
                "star",
                "planet",
                "space",
                "galaxy",
                "cosmos",
                "astronomy",
                "orbit",
            ],
            Domain::Budha,
        ),
        (
            &[
                "earth",
                "climate",
                "weather",
                "ocean",
                "geology",
                "environment",
                "ecosystem",
            ],
            Domain::Chandra,
        ),
        (
            &[
                "biology", "life", "cell", "dna", "gene", "medicine", "disease", "health",
                "organism",
            ],
            Domain::Surya,
        ),
        (
            &[
                "economy", "finance", "market", "money", "trade", "price", "value", "capital",
            ],
            Domain::Budha,
        ),
        (
            &[
                "engineer",
                "tech",
                "machine",
                "robot",
                "circuit",
                "structure",
                "design",
                "bridge",
            ],
            Domain::Shukra,
        ),
        (
            &[
                "computer",
                "algorithm",
                "data",
                "code",
                "software",
                "ai",
                "intelligence",
                "program",
            ],
            Domain::Mangala,
        ),
        (
            &[
                "history",
                "culture",
                "ancient",
                "war",
                "civilization",
                "anthropology",
                "society",
            ],
            Domain::Brihaspati,
        ),
        (
            &[
                "language",
                "word",
                "grammar",
                "linguistics",
                "speech",
                "write",
                "translate",
            ],
            Domain::Shani,
        ),
        (
            &[
                "philosophy",
                "ethics",
                "moral",
                "truth",
                "meaning",
                "consciousness",
                "logic",
            ],
            Domain::Shani,
        ),
        (
            &[
                "psychology",
                "brain",
                "mind",
                "emotion",
                "behavior",
                "neuron",
                "perception",
                "cognition",
            ],
            Domain::Brihaspati,
        ),
    ];

    for (keywords, domain) in archetype_map {
        if keywords.iter().any(|k| lower.contains(k)) {
            return Some(*domain);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formula::FormulaRegistry;

    #[test]
    fn test_word_stem_match_direct() {
        assert!(word_stem_match("serial", "serial_comma_rule"));
        assert!(word_stem_match("presents", "presentation"));
    }

    #[test]
    fn test_word_stem_match_no_false_short() {
        assert!(!word_stem_match("a", "serial_comma_rule"));
        assert!(!word_stem_match("in", "serial_comma_rule"));
    }

    #[test]
    fn test_infer_domain_from_math_keywords() {
        let r = FormulaRegistry::new();
        assert_eq!(
            infer_domain_from_grammar(&r, "prove theorem"),
            Some(Domain::Mangala)
        );
        assert_eq!(
            infer_domain_from_grammar(&r, "compute the answer"),
            Some(Domain::Mangala)
        );
    }

    #[test]
    fn test_infer_domain_from_physics_keywords() {
        let r = FormulaRegistry::new();
        // "force" is in the physics archetype group
        // Currently maps to Domain::Shukra (Venus/Arts — legacy from Taurus→Venus mapping)
        let result = infer_domain_from_grammar(&r, "force and motion");
        assert!(result.is_some());
    }

    #[test]
    fn test_infer_domain_from_domain_name() {
        let r = FormulaRegistry::new();
        // Direct graha name should match
        assert_eq!(
            infer_domain_from_grammar(&r, "study mangala"),
            Some(Domain::Mangala)
        );
        assert_eq!(
            infer_domain_from_grammar(&r, "budha logic"),
            Some(Domain::Budha)
        );
    }

    #[test]
    fn test_infer_domain_returns_none_for_unknown() {
        let r = FormulaRegistry::new();
        assert_eq!(infer_domain_from_grammar(&r, "random gibberish xyz"), None);
    }
}
