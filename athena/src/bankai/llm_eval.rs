//! # LLM formula evaluation — the capstone as a gated KB member
//!
//! `kind = "llm"` formulas hold a prompt template in `expression` instead of
//! evaluable arithmetic. Bankai renders the template with the numeric args,
//! asks the capstone model for a completion, and admits the value **only if
//! it passes the deterministic gates below** — a generation is never trusted
//! raw, and a successful LLM step carries [`LLM_STEP_CONFIDENCE`] instead of
//! the 0.95 a proven math step earns.
//!
//! The render/gate halves are pure and feature-independent so lean builds
//! (no `llm` feature) still compile and test them; only the actual capstone
//! call (in `bankai::mod`) is feature-gated.

use std::collections::HashMap;

use super::BankaiError;
use crate::formula::Formula;

/// Chain-step confidence for a gated LLM output. Deliberately far below the
/// 0.95 of a proven deterministic step: composing two LLM steps should drag
/// a chain's confidence down fast.
pub const LLM_STEP_CONFIDENCE: f64 = 0.6;

/// System prompt for numeric estimation calls. Short on purpose — it is the
/// entire fixed prefill cost per eval on-device.
pub const LLM_EVAL_SYSTEM: &str = "You are Athena's estimator. Reply with a single number only.";

/// Instruction appended after every rendered template so the generation is
/// parseable by [`gate_llm_output`] regardless of how the template is worded.
pub const LLM_NUMBER_INSTRUCTION: &str = "Respond with a single number only.";

/// Render a `kind = "llm"` formula's prompt template against numeric args.
///
/// Every `{input}` placeholder is replaced with the arg's value; all declared
/// inputs must be present (same contract as `EvalEngine::evaluate`) and no
/// unresolved `{...}` placeholder may survive — a typo'd placeholder must
/// fail loudly here, not reach the model as literal braces.
pub fn render_prompt(
    formula: &Formula,
    args: &HashMap<String, f64>,
) -> Result<String, BankaiError> {
    let mut prompt = formula.expression.clone();
    for input in &formula.inputs {
        let value = args.get(input).ok_or_else(|| {
            BankaiError::EvalError(format!(
                "missing argument: '{}' for formula '{}'",
                input, formula.id
            ))
        })?;
        prompt = prompt.replace(&format!("{{{}}}", input), &format_number(*value));
    }

    if let Some(start) = prompt.find('{') {
        let tail: String = prompt[start..].chars().take(24).collect();
        return Err(BankaiError::EvalError(format!(
            "unresolved placeholder in llm formula '{}': {}...",
            formula.id, tail
        )));
    }

    Ok(format!("{}\n\n{}", prompt, LLM_NUMBER_INSTRUCTION))
}

/// Format an f64 for prompt text: integers without the trailing `.0` noise.
fn format_number(v: f64) -> String {
    if v.fract() == 0.0 && v.abs() < 1e15 {
        format!("{}", v as i64)
    } else {
        format!("{}", v)
    }
}

/// The gate on raw LLM output: admit the value only if the generation
/// contains **exactly one** numeric token, and that token is finite.
///
/// Strictness is the point — "around 3 or 4 minutes" is ambiguous and must
/// be rejected, not resolved by picking one. Reject-on-ambiguity keeps the
/// admitted value attributable to the model rather than to parsing luck.
pub fn gate_llm_output(raw: &str, formula_id: &str) -> Result<f64, BankaiError> {
    let numbers: Vec<f64> = raw
        .split(|c: char| c.is_whitespace() || c == '`' || c == '*')
        .map(|tok| tok.trim_matches(|c: char| !c.is_ascii_digit() && c != '.' && c != '-'))
        .filter(|tok| !tok.is_empty())
        .filter_map(|tok| tok.parse::<f64>().ok())
        .collect();

    match numbers.as_slice() {
        [] => Err(BankaiError::EvalError(format!(
            "llm gate rejected '{}': no numeric value in output: {:?}",
            formula_id,
            truncate(raw, 80)
        ))),
        [value] if value.is_finite() => Ok(*value),
        [value] => Err(BankaiError::EvalError(format!(
            "llm gate rejected '{}': non-finite value {}",
            formula_id, value
        ))),
        many => Err(BankaiError::EvalError(format!(
            "llm gate rejected '{}': ambiguous output, {} numeric values: {:?}",
            formula_id,
            many.len(),
            truncate(raw, 80)
        ))),
    }
}

fn truncate(s: &str, max: usize) -> String {
    let t = s.trim();
    if t.chars().count() <= max {
        t.to_string()
    } else {
        let cut: String = t.chars().take(max).collect();
        format!("{}...", cut)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formula::{Formula, FormulaType};
    use crate::wheel::Domain;

    fn llm_formula() -> Formula {
        let mut f = Formula::new(
            "estimate_reading_minutes",
            FormulaType::Llm,
            Domain::Budha,
            vec!["word_count"],
            "reading_minutes",
            "Estimate how many minutes an average adult needs to read {word_count} words.",
            "LLM-estimated reading time",
        );
        f.evidence = None;
        f
    }

    #[test]
    fn test_render_substitutes_and_appends_instruction() {
        let mut args = HashMap::new();
        args.insert("word_count".to_string(), 1200.0);
        let prompt = render_prompt(&llm_formula(), &args).unwrap();
        assert!(prompt.contains("read 1200 words"));
        assert!(prompt.ends_with(LLM_NUMBER_INSTRUCTION));
        assert!(!prompt.contains('{'));
    }

    #[test]
    fn test_render_missing_arg_fails() {
        let args = HashMap::new();
        assert!(render_prompt(&llm_formula(), &args).is_err());
    }

    #[test]
    fn test_render_unresolved_placeholder_fails() {
        let mut f = llm_formula();
        f.expression = "Estimate {word_count} words at {wpm} wpm.".to_string();
        let mut args = HashMap::new();
        args.insert("word_count".to_string(), 1200.0);
        // {wpm} is not a declared input, so it survives substitution → reject
        let err = render_prompt(&f, &args).unwrap_err();
        assert!(err.to_string().contains("unresolved placeholder"));
    }

    #[test]
    fn test_gate_accepts_single_number() {
        assert_eq!(gate_llm_output("6", "f").unwrap(), 6.0);
        assert_eq!(gate_llm_output("About 6 minutes.", "f").unwrap(), 6.0);
        assert_eq!(gate_llm_output("**4.5**", "f").unwrap(), 4.5);
        assert_eq!(gate_llm_output("-12.25\n", "f").unwrap(), -12.25);
    }

    #[test]
    fn test_gate_rejects_no_number() {
        assert!(gate_llm_output("I cannot estimate that.", "f").is_err());
        assert!(gate_llm_output("", "f").is_err());
    }

    #[test]
    fn test_gate_rejects_ambiguous_output() {
        let err = gate_llm_output("around 3 or 4 minutes", "f").unwrap_err();
        assert!(err.to_string().contains("ambiguous"));
    }
}
