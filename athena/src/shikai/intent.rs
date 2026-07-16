//! # Intent identification for Shikai
//!
//! Defines the `Intent` enum and the `identify_intent` function that classifies
//! a user's natural language or structured query into one of six intent categories.

use serde::{Deserialize, Serialize};

use crate::entity::EntityRegistry;
use crate::formula::FormulaRegistry;

use super::grammar::infer_domain_from_grammar;

/// The intent of a query — what the user wants to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Intent {
    /// Evaluate an expression or formula
    Evaluate,
    /// Validate a claim or calculation
    Validate,
    /// Traverse the wheel graph
    Traverse,
    /// Compose formulas into a chain
    Compose,
    /// Search the formula database
    Search,
    /// Get information about the system
    Info,
}

/// Identify the intent of a query string.
///
/// Examines the query text for keywords and patterns to determine whether the
/// user wants to evaluate, validate, traverse, compose, search, or get info.
///
/// Takes `&FormulaRegistry` and `&EntityRegistry` to support fallback matching
/// against known formulas, entities, and grammar rules when no explicit keyword
/// is found.
///
/// `nlp_hint` is an optional intent from the Zanpakuto NLP preprocessing layer.
/// If the NLP engine has high confidence (>0.85), it's used as the primary signal.
/// Otherwise, explicit keywords in the query take priority and NLP serves as fallback.
#[allow(clippy::if_same_then_else)]
pub fn identify_intent(
    query: &str,
    formulas: &FormulaRegistry,
    entities: &EntityRegistry,
    nlp_hint: Option<Intent>,
) -> Result<Intent, String> {
    let lower = query.to_lowercase();

    // ── Phase 1: Explicit keyword match (always wins) ─────────────────
    // These are unambiguous commands like "validate", "traverse", etc.

    if lower.starts_with("validate") || lower.starts_with("check") || lower.contains("verify") {
        Ok(Intent::Validate)
    } else if lower.starts_with("traverse")
        || lower.starts_with("explore")
        || lower.starts_with("follow")
    {
        Ok(Intent::Traverse)
    } else if lower.starts_with("compose")
        || lower.starts_with("chain")
        || lower.contains("connect")
    {
        Ok(Intent::Compose)
    } else if lower.starts_with("search")
        || lower.starts_with("find")
        || lower.starts_with("lookup")
    {
        Ok(Intent::Search)
    } else if lower.starts_with("info") || lower.starts_with("help") || lower.starts_with("status")
    {
        Ok(Intent::Info)
    } else if lower.starts_with("solve")
        || lower.starts_with("calculate")
        || lower.starts_with("eval")
    {
        Ok(Intent::Evaluate)
    } else if lower.contains('=')
        || lower.contains('+')
        || lower.contains('-')
        || lower.contains('*')
        || lower.contains('/')
    {
        Ok(Intent::Evaluate)
    } else {
        // ── Phase 2: NLP hint (for ambiguous queries) ────────────────
        // If NLP has high confidence, use it directly. This catches queries
        // like "search momentum" where "search" is not at the start but NLP
        // recognizes the intent from deeper analysis.
        if let Some(hint) = nlp_hint {
            // NLP scores are 0.0–1.0; if the hint made it through as likely_intent,
            // it means the NLP engine found sufficient evidence (>= other intents).
            return Ok(hint);
        }

        // ── Phase 3: Fallback heuristics ─────────────────────────────
        // Default: try to find a matching formula
        let results = formulas.search(query);
        if !results.is_empty() {
            return Ok(Intent::Evaluate);
        }
        // Check for entity mentions
        if resolve_entity_context(query, entities).is_some() {
            return Ok(Intent::Evaluate);
        }
        // Check for grammar rule matches
        if infer_domain_from_grammar(formulas, query).is_some() {
            return Ok(Intent::Evaluate);
        }
        // Fallback: significant-word heuristic
        if query.split_whitespace().any(|w| {
            let cleaned = w.trim_matches(|c: char| !c.is_alphanumeric());
            cleaned.len() > 4
                && !matches!(
                    cleaned.to_lowercase().as_str(),
                    "about"
                        | "after"
                        | "also"
                        | "another"
                        | "around"
                        | "because"
                        | "been"
                        | "before"
                        | "being"
                        | "below"
                        | "between"
                        | "both"
                        | "come"
                        | "could"
                        | "doing"
                        | "down"
                        | "each"
                        | "even"
                        | "find"
                        | "first"
                        | "follow"
                        | "from"
                        | "give"
                        | "great"
                        | "have"
                        | "having"
                        | "here"
                        | "into"
                        | "just"
                        | "know"
                        | "like"
                        | "little"
                        | "long"
                        | "make"
                        | "many"
                        | "might"
                        | "more"
                        | "most"
                        | "much"
                        | "need"
                        | "only"
                        | "over"
                        | "place"
                        | "right"
                        | "said"
                        | "same"
                        | "should"
                        | "show"
                        | "some"
                        | "still"
                        | "such"
                        | "tell"
                        | "than"
                        | "that"
                        | "their"
                        | "them"
                        | "then"
                        | "there"
                        | "these"
                        | "they"
                        | "think"
                        | "this"
                        | "those"
                        | "through"
                        | "under"
                        | "until"
                        | "upon"
                        | "used"
                        | "very"
                        | "want"
                        | "well"
                        | "were"
                        | "what"
                        | "when"
                        | "where"
                        | "which"
                        | "while"
                        | "will"
                        | "with"
                        | "without"
                        | "word"
                        | "would"
                        | "your"
                )
        }) {
            Ok(Intent::Evaluate)
        } else {
            Err(format!("unrecognized intent: {}", query))
        }
    }
}

/// Resolve an entity mention from a query string.
///
/// Checks if any whitespace-delimited word matches an entity ID in the registry.
/// Returns the entity ID if exactly one entity is found.
pub(crate) fn resolve_entity_context(query: &str, entities: &EntityRegistry) -> Option<String> {
    // First: exact seed ID match on any word
    for word in query.split_whitespace() {
        let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
        if !cleaned.is_empty() && entities.get_seed(cleaned).is_some() {
            return Some(cleaned.to_string());
        }
    }
    // Second: case-insensitive seed name match
    for word in query.split_whitespace() {
        let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
        if !cleaned.is_empty() {
            let results = entities.search_seeds(cleaned);
            if results.len() == 1 {
                return Some(results[0].id.clone());
            }
        }
    }
    // Third: runtime entity text match
    for word in query.split_whitespace() {
        let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
        if !cleaned.is_empty() {
            let results = entities.search(cleaned);
            if results.len() == 1 {
                return Some(results[0].id.clone());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::EntityRegistry;
    use crate::formula::{Formula, FormulaRegistry};
    use crate::wheel::Domain;

    fn empty_registry() -> FormulaRegistry {
        FormulaRegistry::new()
    }

    #[test]
    fn test_identify_intent_evaluate() {
        let reg = empty_registry();
        let ent = EntityRegistry::new();
        assert_eq!(
            identify_intent("2 + 2", &reg, &ent, None).unwrap(),
            Intent::Evaluate
        );
        assert_eq!(
            identify_intent("mass * acceleration", &reg, &ent, None).unwrap(),
            Intent::Evaluate
        );
    }

    #[test]
    fn test_identify_intent_solve() {
        let reg = empty_registry();
        let ent = EntityRegistry::new();
        assert_eq!(
            identify_intent("solve newtons_second", &reg, &ent, None).unwrap(),
            Intent::Evaluate
        );
    }

    #[test]
    fn test_identify_intent_validate() {
        let reg = empty_registry();
        let ent = EntityRegistry::new();
        assert_eq!(
            identify_intent("validate 2 + 2 = 4", &reg, &ent, None).unwrap(),
            Intent::Validate
        );
        assert_eq!(
            identify_intent("check F = ma", &reg, &ent, None).unwrap(),
            Intent::Validate
        );
    }

    #[test]
    fn test_identify_intent_traverse() {
        let reg = empty_registry();
        let ent = EntityRegistry::new();
        assert_eq!(
            identify_intent("traverse from aries", &reg, &ent, None).unwrap(),
            Intent::Traverse
        );
    }

    #[test]
    fn test_identify_intent_search() {
        let reg = empty_registry();
        let ent = EntityRegistry::new();
        assert_eq!(
            identify_intent("search momentum", &reg, &ent, None).unwrap(),
            Intent::Search
        );
    }

    #[test]
    fn test_identify_intent_info() {
        let reg = empty_registry();
        let ent = EntityRegistry::new();
        assert_eq!(
            identify_intent("info", &reg, &ent, None).unwrap(),
            Intent::Info
        );
        assert_eq!(
            identify_intent("help", &reg, &ent, None).unwrap(),
            Intent::Info
        );
    }

    #[test]
    fn test_fallback_topic_as_evaluate() {
        let reg = empty_registry();
        let ent = EntityRegistry::new();
        // Topics that don't match any formula/entity/grammar should still
        // resolve to Evaluate via the significant-word fallback
        assert_eq!(
            identify_intent("advanced calculus", &reg, &ent, None).unwrap(),
            Intent::Evaluate
        );
        assert_eq!(
            identify_intent("quantum mechanics", &reg, &ent, None).unwrap(),
            Intent::Evaluate
        );
    }

    #[test]
    fn test_unrecognized_intent() {
        let reg = empty_registry();
        let ent = EntityRegistry::new();
        // Short/gibberish queries should fail
        assert!(identify_intent("a b c", &reg, &ent, None).is_err());
        assert!(identify_intent("foo bar baz", &reg, &ent, None).is_err());
    }

    #[test]
    fn test_formula_match_triggers_evaluate() {
        let mut reg = FormulaRegistry::new();
        reg.register_all(vec![Formula::atomic(
            "pythagorean",
            Domain::Mangala,
            vec!["a", "b"],
            "c",
            "(a^2 + b^2).sqrt()",
            "Pythagorean theorem",
        )])
        .unwrap();
        let ent = EntityRegistry::new();
        assert_eq!(
            identify_intent("pythagorean", &reg, &ent, None).unwrap(),
            Intent::Evaluate
        );
    }

    #[test]
    fn test_nlp_hint_overrides_fallback() {
        let reg = empty_registry();
        let ent = EntityRegistry::new();
        // Without NLP hint, "momentum force" falls to significant-word → Evaluate.
        // With a Search hint, it should resolve to Search.
        assert_eq!(
            identify_intent("momentum force", &reg, &ent, Some(Intent::Search)).unwrap(),
            Intent::Search
        );
    }

    #[test]
    fn test_nlp_hint_does_not_override_explicit_keyword() {
        let reg = empty_registry();
        let ent = EntityRegistry::new();
        // Explicit "validate" at start should win over a contradictory NLP hint
        assert_eq!(
            identify_intent("validate 2 + 2", &reg, &ent, Some(Intent::Evaluate)).unwrap(),
            Intent::Validate
        );
    }

    #[test]
    fn test_resolve_entity_context() {
        let mut entities = EntityRegistry::new();
        entities.register_seed(crate::entity::SeedEntity {
            id: "earth".into(),
            name: "Earth".into(),
            description: "Planet Earth".into(),
            properties: [("mass".into(), 5.972e24)].into(),
            ..Default::default()
        });
        assert_eq!(
            resolve_entity_context("earth mass", &entities),
            Some("earth".to_string())
        );
        assert_eq!(resolve_entity_context("unknown", &entities), None);
    }
}
