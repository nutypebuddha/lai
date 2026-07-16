pub mod core;
pub mod economy;
pub mod gates;
pub mod inference;
pub mod kb;
pub mod mcp;
pub mod output;
pub mod state;
pub mod tanto;

pub use core::ball::{Ball, GateResult, TokenCandidate};
pub use core::pin::{Gate, Pin, PinField};
pub use core::pocket::Pocket;
pub use economy::budget::Budget;
pub use economy::conversation::{ConversationTracker, DailyLimits, UsageReport};
pub use economy::cost::CostTracker;
pub use economy::tray::BallEconomy;
pub use gates::{ConfidenceGate, FactGate, FallacyGate, FormalGate, LogicGate, MathGate};
pub use inference::{
    CacheHit, CacheStats, CidError, CidResult, CompressionLevel, CompressionStats, GateScore,
    InferenceConfig, InferenceEngine, Pipeline, PromptCompressor, ProxyConfig, ProxyRequest,
    QualityReport, ResponseScorer, SemanticCache, SuggestedAction, TokenFix, TokenFixer,
    ValidationRequest, ValidationResult,
};
pub use kb::facts::{Fact, KnowledgeBase};
pub use state::machine::{State, StateMachine, ValidationDepth};
