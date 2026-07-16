//! # TokenBudget — self-contained token spend tracking
//!
//! Every LLM call produces a `TokenSpend` record. The budget tracks
//! cumulative usage against configurable caps. Spends are stored
//! internally — no dependency on the entity registry.
//!
//! ```text
//! "how many tokens did we spend on Pisces disambiguation?"
//!   → filter spends where domain = Pisces, sum prompt_tokens
//!
//! "what's the most expensive query this session?"
//!   → spends sorted by total_tokens descending
//! ```

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::wheel::Domain;

/// Default token budget limits.
pub const DEFAULT_MAX_PROMPT_TOKENS: usize = 2000;
pub const DEFAULT_MAX_COMPLETION_TOKENS: usize = 1000;
pub const DEFAULT_MAX_TOTAL_TOKENS: usize = 3000;

/// A single LLM token spend record.
///
/// Every LLM call creates one of these.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSpend {
    /// Unique identifier (e.g. "tok_001")
    pub id: String,
    /// Domain this spend is grounded on (the query's domain)
    pub domain: Domain,
    /// The prompt that was sent
    pub prompt_snippet: String,
    /// Estimated prompt tokens
    pub prompt_tokens: usize,
    /// Completion tokens received
    pub completion_tokens: usize,
    /// Total tokens for this call
    pub total_tokens: usize,
    /// Unix timestamp
    pub timestamp: u64,
    /// What this call was for (e.g. "disambiguation", "translation", "expansion")
    pub purpose: String,
    /// The entity ID that triggered this spend (if any)
    pub source_entity: Option<String>,
}

/// Reason for blocking an LLM call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BlockReason {
    PromptLimit,
    CompletionLimit,
    TotalLimit,
    AlreadyExceeded,
}

/// Result of a budget check.
#[derive(Debug, Clone, Serialize)]
pub enum BudgetCheck {
    /// Within budget — proceed.
    Ok,
    /// Warning — approaching limit.
    Warning { prompt_pct: f64, total_pct: f64 },
    /// Exceeded — blocked.
    Blocked {
        reason: BlockReason,
        message: String,
    },
}

/// The token budget — self-contained spend tracking.
///
/// Spends are stored internally. No dependency on entity registry.
#[derive(Debug, Clone)]
pub struct TokenBudget {
    /// Limits
    max_prompt: usize,
    max_completion: usize,
    max_total: usize,
    /// Whether the budget has been exceeded (stops further calls)
    exceeded: bool,
    /// Running counter for generating unique token spend IDs
    next_id: u64,
    /// Internal list of recorded spends
    spends: Vec<TokenSpend>,
}

impl Default for TokenBudget {
    fn default() -> Self {
        Self::new(
            DEFAULT_MAX_PROMPT_TOKENS,
            DEFAULT_MAX_COMPLETION_TOKENS,
            DEFAULT_MAX_TOTAL_TOKENS,
        )
    }
}

impl TokenBudget {
    /// Create a new token budget with the given caps.
    pub fn new(max_prompt: usize, max_completion: usize, max_total: usize) -> Self {
        TokenBudget {
            max_prompt,
            max_completion,
            max_total,
            exceeded: false,
            next_id: 1,
            spends: Vec::new(),
        }
    }

    /// Estimate the number of tokens in a text string.
    ///
    /// Conservative heuristic: ~4 chars/token for NL, ~2 chars/token for code.
    /// Adds a 10% safety margin.
    pub fn estimate_tokens(text: &str, is_code: bool) -> usize {
        let chars = text.len() as f64;
        let rate = if is_code { 2.0 } else { 4.0 };
        ((chars / rate) * 1.1).ceil() as usize
    }

    /// Check whether a prompt fits within the remaining budget.
    pub fn check_prompt(&self, prompt: &str, is_code: bool) -> BudgetCheck {
        if self.exceeded {
            return BudgetCheck::Blocked {
                reason: BlockReason::AlreadyExceeded,
                message: "budget already exceeded — reset or increase limits".into(),
            };
        }
        let estimated = Self::estimate_tokens(prompt, is_code);
        let (prompt_used, total_used) = self.compute_usage();

        let new_prompt = prompt_used + estimated;
        if new_prompt > self.max_prompt {
            return BudgetCheck::Blocked {
                reason: BlockReason::PromptLimit,
                message: format!(
                    "prompt token limit: {}/{} would exceed max {}",
                    new_prompt, self.max_prompt, self.max_prompt
                ),
            };
        }

        // Estimate worst-case: prompt + max possible completion
        let new_total = total_used + estimated + self.max_completion;
        if new_total > self.max_total {
            return BudgetCheck::Blocked {
                reason: BlockReason::TotalLimit,
                message: format!(
                    "total token limit: would reach {} vs max {}",
                    new_total, self.max_total
                ),
            };
        }

        let prompt_pct = new_prompt as f64 / self.max_prompt as f64;
        let total_pct = (total_used + estimated) as f64 / self.max_total as f64;

        if prompt_pct > 0.8 || total_pct > 0.8 {
            BudgetCheck::Warning {
                prompt_pct,
                total_pct,
            }
        } else {
            BudgetCheck::Ok
        }
    }

    /// Record a token spend.
    ///
    /// Returns the token spend ID on success.
    #[allow(clippy::too_many_arguments)]
    pub fn record_spend(
        &mut self,
        prompt: &str,
        prompt_tokens: usize,
        completion_tokens: usize,
        domain: Domain,
        purpose: &str,
        source_entity: Option<&str>,
        _is_code: bool,
    ) -> Result<String, BudgetCheck> {
        // Check the declared token counts against the caps. (`check_prompt`
        // is for pre-flight string estimates; here we have actual numbers.)
        if self.exceeded {
            return Err(BudgetCheck::Blocked {
                reason: BlockReason::AlreadyExceeded,
                message: "budget already exceeded — reset or increase limits".into(),
            });
        }
        let (prompt_used, total_used) = self.compute_usage();
        let completion_used = total_used.saturating_sub(prompt_used);
        if prompt_used + prompt_tokens > self.max_prompt {
            return Err(BudgetCheck::Blocked {
                reason: BlockReason::PromptLimit,
                message: format!(
                    "prompt token limit: {} would exceed max {}",
                    prompt_used + prompt_tokens,
                    self.max_prompt
                ),
            });
        }
        if completion_used + completion_tokens > self.max_completion {
            return Err(BudgetCheck::Blocked {
                reason: BlockReason::CompletionLimit,
                message: format!(
                    "completion token limit: {} would exceed max {}",
                    completion_used + completion_tokens,
                    self.max_completion
                ),
            });
        }
        if total_used + prompt_tokens + completion_tokens > self.max_total {
            return Err(BudgetCheck::Blocked {
                reason: BlockReason::TotalLimit,
                message: format!(
                    "total token limit: {} would exceed max {}",
                    total_used + prompt_tokens + completion_tokens,
                    self.max_total
                ),
            });
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let total = prompt_tokens + completion_tokens;
        let id = format!("tok_{:03}", self.next_id);
        self.next_id += 1;

        let prompt_snippet = if prompt.len() > 60 {
            format!("{}...", &prompt[..60])
        } else {
            prompt.to_string()
        };

        let spend = TokenSpend {
            id: id.clone(),
            domain,
            prompt_snippet,
            prompt_tokens,
            completion_tokens,
            total_tokens: total,
            timestamp: now,
            purpose: purpose.to_string(),
            source_entity: source_entity.map(String::from),
        };

        self.spends.push(spend);

        // Update exceeded flag
        let (prompt_used, total_used) = self.compute_usage();
        if prompt_used > self.max_prompt || total_used > self.max_total {
            self.exceeded = true;
        }

        Ok(id)
    }

    /// Compute current usage from internal spend records.
    pub fn compute_usage(&self) -> (usize, usize) {
        let mut prompt_used = 0usize;
        let mut total_used = 0usize;
        for spend in &self.spends {
            prompt_used += spend.prompt_tokens;
            total_used += spend.total_tokens;
        }
        (prompt_used, total_used)
    }

    /// Get usage statistics.
    pub fn stats(&self) -> BudgetStats {
        let (prompt_used, total_used) = self.compute_usage();
        let completion_used = total_used.saturating_sub(prompt_used);

        let spends: Vec<TokenSpendInfo> = self
            .spends
            .iter()
            .map(|s| TokenSpendInfo {
                id: s.id.clone(),
                domain: s.domain,
                purpose: s.purpose.clone(),
                prompt_tokens: s.prompt_tokens,
                completion_tokens: s.completion_tokens,
                total_tokens: s.total_tokens,
                timestamp: s.timestamp,
            })
            .collect();

        BudgetStats {
            prompt_used,
            completion_used,
            total_used,
            max_prompt: self.max_prompt,
            max_completion: self.max_completion,
            max_total: self.max_total,
            exceeded: self.exceeded,
            prompt_pct: if self.max_prompt > 0 {
                prompt_used as f64 / self.max_prompt as f64
            } else {
                0.0
            },
            completion_pct: if self.max_completion > 0 {
                completion_used as f64 / self.max_completion as f64
            } else {
                0.0
            },
            total_pct: if self.max_total > 0 {
                total_used as f64 / self.max_total as f64
            } else {
                0.0
            },
            spends,
        }
    }

    /// Get all token spends.
    pub fn spends(&self) -> &[TokenSpend] {
        &self.spends
    }

    /// Reset the exceeded flag (does not clear existing spends).
    pub fn reset(&mut self) {
        self.exceeded = false;
    }

    /// Whether the budget has been exceeded.
    pub fn is_exceeded(&self) -> bool {
        self.exceeded
    }
}

/// Token spend info for display.
#[derive(Debug, Clone, Serialize)]
pub struct TokenSpendInfo {
    pub id: String,
    pub domain: Domain,
    pub purpose: String,
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
    pub timestamp: u64,
}

/// Budget usage statistics.
#[derive(Debug, Clone, Serialize)]
pub struct BudgetStats {
    pub prompt_used: usize,
    pub completion_used: usize,
    pub total_used: usize,
    pub max_prompt: usize,
    pub max_completion: usize,
    pub max_total: usize,
    pub exceeded: bool,
    pub prompt_pct: f64,
    pub completion_pct: f64,
    pub total_pct: f64,
    pub spends: Vec<TokenSpendInfo>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budget_new() {
        let b = TokenBudget::new(100, 50, 150);
        assert!(!b.is_exceeded());
        assert_eq!(b.compute_usage(), (0, 0));
    }

    #[test]
    fn test_record_spend() {
        let mut b = TokenBudget::new(1000, 500, 1500);
        let result = b.record_spend("test prompt", 50, 30, Domain::Mangala, "test", None, false);
        assert!(result.is_ok());
        assert_eq!(b.compute_usage(), (50, 80));
    }

    #[test]
    fn test_budget_exceeded() {
        let mut b = TokenBudget::new(100, 50, 150);
        let result = b.record_spend("big prompt", 120, 40, Domain::Mangala, "test", None, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_budget_reset() {
        let mut b = TokenBudget::new(100, 50, 150);
        b.exceeded = true;
        assert!(b.is_exceeded());
        b.reset();
        assert!(!b.is_exceeded());
    }

    #[test]
    fn test_budget_stats() {
        let mut b = TokenBudget::new(1000, 500, 1500);
        b.record_spend("test", 100, 50, Domain::Shukra, "physics", None, false)
            .unwrap();
        let stats = b.stats();
        assert_eq!(stats.prompt_used, 100);
        assert_eq!(stats.completion_used, 50);
        assert_eq!(stats.total_used, 150);
        assert_eq!(stats.spends.len(), 1);
    }

    #[test]
    fn test_estimate_tokens() {
        let nl = TokenBudget::estimate_tokens("hello world", false);
        assert!(nl > 0);
        let code = TokenBudget::estimate_tokens("fn main() { println!(\"hi\"); }", true);
        assert!(code > 0);
    }

    #[test]
    fn test_budget_multiple_spends() {
        let mut b = TokenBudget::new(200, 100, 300);
        b.record_spend("first", 50, 25, Domain::Mangala, "test1", None, false)
            .unwrap();
        b.record_spend("second", 50, 25, Domain::Shukra, "test2", None, false)
            .unwrap();
        assert_eq!(b.compute_usage(), (100, 150));
        assert_eq!(b.spends().len(), 2);
    }

    #[test]
    fn test_budget_over_total_limit() {
        let mut b = TokenBudget::new(100, 50, 120);
        // 50 prompt + 50 completion = 100 total, OK
        b.record_spend("first", 50, 25, Domain::Mangala, "test", None, false)
            .unwrap();
        // Next: 50 prompt + 25 completion + 50 estimated completion = would exceed 120 total
        let result = b.record_spend("second", 50, 25, Domain::Shukra, "test", None, false);
        assert!(result.is_err()); // Would exceed total limit
    }
}
