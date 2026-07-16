//! # Distill — the registry teaches the capstone
//!
//! Generates (query → intent) training pairs for fine-tuning the LLM
//! copilot, straight from the formula/entity registries. Every label is
//! provably correct: the deterministic engine is the ground truth, so the
//! KB generates its own curriculum — no human annotation, no other model.
//!
//! Labels are serialized from [`RouterIntent`] itself, so the training
//! target and the schema `LlmRouter::route` parses can never drift apart.
//!
//! Deterministic by construction (repo convention: no randomness): pair
//! order follows domain wheel order, then formula id sort; template choice
//! cycles by index.

use crate::bankai::llm_router::{QueryCategory, RouterIntent, RouterToken};
use crate::entity::EntityRegistry;
use crate::formula::FormulaRegistry;
use crate::wheel::ALL_DOMAINS;
use serde::Serialize;

/// System prompt baked into the fine-tune. Deliberately short: after
/// training, this is the entire fixed prefill cost per routing call.
pub const DISTILL_SYSTEM_PROMPT: &str =
    "You are Athena's router. Reply with a single JSON intent object.";

/// One chat-format training example (JSONL-ready).
#[derive(Debug, Serialize)]
pub struct TrainingPair {
    pub messages: [Message; 3],
}

/// A chat message in the training example.
#[derive(Debug, Serialize)]
pub struct Message {
    pub role: &'static str,
    pub content: String,
}

impl TrainingPair {
    fn new(query: String, intent: &RouterIntent) -> Self {
        Self {
            messages: [
                Message {
                    role: "system",
                    content: DISTILL_SYSTEM_PROMPT.to_string(),
                },
                Message {
                    role: "user",
                    content: query,
                },
                Message {
                    role: "assistant",
                    content: serde_json::to_string(intent).expect("RouterIntent serializes"),
                },
            ],
        }
    }
}

/// Replace underscores for natural-language templating.
fn humanize(ident: &str) -> String {
    ident.replace('_', " ")
}

/// Join variable names as natural language: "a, b and c".
fn join_vars(vars: &[String]) -> String {
    let human: Vec<String> = vars.iter().map(|v| humanize(v)).collect();
    match human.len() {
        0 => String::new(),
        1 => human[0].clone(),
        _ => format!(
            "{} and {}",
            human[..human.len() - 1].join(", "),
            human[human.len() - 1]
        ),
    }
}

/// Tokens for a formula's variables, grounded in its domain.
fn formula_tokens(vars: &[&str], domain_name: &str) -> Vec<RouterToken> {
    vars.iter()
        .map(|v| RouterToken {
            text: humanize(v),
            domain: Some(domain_name.to_lowercase()),
            graha: Some(domain_name.to_string()),
            sign: None,
            aspect_to_next: None,
            mass: 1.0,
        })
        .collect()
}

/// Generate the full training set from the registries.
pub fn distill(registry: &FormulaRegistry, entities: &EntityRegistry) -> Vec<TrainingPair> {
    let mut pairs = Vec::new();

    // ── Formula-grounded pairs: Evaluate / Search / Reason / Validate ──
    for domain in ALL_DOMAINS {
        let mut formulas = registry.by_domain(domain);
        formulas.sort_by(|a, b| a.id.cmp(&b.id));
        for (i, f) in formulas.iter().enumerate() {
            if f.inputs.is_empty() {
                continue;
            }
            let domain_name = domain.name();
            let inputs = join_vars(&f.inputs);
            let output = humanize(&f.output);
            let mut vars: Vec<&str> = f.inputs.iter().map(|s| s.as_str()).collect();
            vars.push(f.output.as_str());
            let tokens = formula_tokens(&vars, domain_name);
            let explanation = format!("{}: {} = {}", f.id, f.output, f.expression);

            let intent = |category: QueryCategory| RouterIntent {
                category,
                tokens: tokens.clone(),
                suggested_chain: vec![f.id.clone()],
                suggested_entities: Vec::new(),
                confidence: 0.95,
                deterministic_only: true,
                explanation: explanation.clone(),
            };

            pairs.push(TrainingPair::new(
                format!("what is {} given {}?", output, inputs),
                &intent(QueryCategory::Evaluate),
            ));
            pairs.push(TrainingPair::new(
                format!("which formula relates {} to {}?", inputs, output),
                &intent(QueryCategory::Search),
            ));
            pairs.push(TrainingPair::new(
                format!("how do I derive {} from {}?", output, inputs),
                &intent(QueryCategory::Reason),
            ));
            // Every 4th formula also trains Validate (cycled, deterministic)
            if i % 4 == 0 {
                pairs.push(TrainingPair::new(
                    format!("is '{}' the right way to get {}?", f.expression, output),
                    &intent(QueryCategory::Validate),
                ));
            }
        }
    }

    // ── Entity-grounded pairs ──
    for domain in ALL_DOMAINS {
        let mut seeds = entities.seeds_by_graha(domain);
        seeds.sort_by(|a, b| a.id.cmp(&b.id));
        for seed in seeds {
            let short_desc: String = seed.description.chars().take(80).collect();
            let intent = RouterIntent {
                category: QueryCategory::Entity,
                tokens: vec![RouterToken {
                    text: seed.name.clone(),
                    domain: Some(domain.name().to_lowercase()),
                    graha: Some(domain.name().to_string()),
                    sign: None,
                    aspect_to_next: None,
                    mass: 1.0,
                }],
                suggested_chain: Vec::new(),
                suggested_entities: vec![seed.id.clone()],
                confidence: 0.95,
                deterministic_only: true,
                explanation: short_desc,
            };
            pairs.push(TrainingPair::new(
                format!("tell me about {}", seed.name),
                &intent,
            ));
        }
    }

    // ── Traverse pairs: every ordered domain pair on the wheel ──
    for from in ALL_DOMAINS {
        for to in ALL_DOMAINS {
            if from == to {
                continue;
            }
            let intent = RouterIntent {
                category: QueryCategory::Traverse,
                tokens: formula_tokens(&[], from.name()),
                suggested_chain: Vec::new(),
                suggested_entities: Vec::new(),
                confidence: 0.95,
                deterministic_only: true,
                explanation: format!("traverse {} -> {}", from.name(), to.name()),
            };
            pairs.push(TrainingPair::new(
                format!(
                    "find a path from {} to {}",
                    from.name().to_lowercase(),
                    to.name().to_lowercase()
                ),
                &intent,
            ));
        }
    }

    // ── Info pairs ──
    for q in ["what can you do?", "who are you?", "show system info"] {
        let intent = RouterIntent {
            category: QueryCategory::Info,
            tokens: Vec::new(),
            suggested_chain: Vec::new(),
            suggested_entities: Vec::new(),
            confidence: 0.95,
            deterministic_only: true,
            explanation: "system information request".to_string(),
        };
        pairs.push(TrainingPair::new(q.to_string(), &intent));
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formula::Formula;
    use crate::wheel::Domain;

    fn tiny_registry() -> FormulaRegistry {
        let mut r = FormulaRegistry::new();
        r.register(Formula::atomic(
            "kinetic_energy",
            Domain::Shukra,
            vec!["mass", "velocity"],
            "kinetic_energy",
            "0.5 * mass * velocity^2",
            "kinetic energy",
        ))
        .unwrap();
        r
    }

    #[test]
    fn test_distill_produces_parseable_labels() {
        let pairs = distill(&tiny_registry(), &EntityRegistry::new());
        // 3 formula templates + 1 validate (i=0) + 72 traverse + 3 info
        assert!(pairs.len() >= 79, "got {}", pairs.len());
        // Every assistant message must round-trip through the router's
        // own intent type — the whole point of distilling from the struct.
        for p in &pairs {
            let intent: RouterIntent = serde_json::from_str(&p.messages[2].content).unwrap();
            assert!(intent.confidence > 0.0);
        }
    }

    #[test]
    fn test_distill_is_deterministic() {
        let r = tiny_registry();
        let e = EntityRegistry::new();
        let a = serde_json::to_string(&distill(&r, &e).first().map(|p| &p.messages)).unwrap();
        let b = serde_json::to_string(&distill(&r, &e).first().map(|p| &p.messages)).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_formula_queries_route_to_formula_id() {
        let pairs = distill(&tiny_registry(), &EntityRegistry::new());
        let eval_pair = &pairs[0];
        assert!(eval_pair.messages[1].content.contains("kinetic energy"));
        let intent: RouterIntent = serde_json::from_str(&eval_pair.messages[2].content).unwrap();
        assert_eq!(intent.suggested_chain, vec!["kinetic_energy"]);
    }
}
