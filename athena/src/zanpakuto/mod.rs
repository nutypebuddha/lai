//! # Zanpakuto — the access control layer
//!
//! In the Bleach universe, giving a name to an Asauchi makes it a Zanpakuto — it becomes
//! *yours*. The Zanpakuto knows its wielder and responds only to them.
//!
//! In Athena, this is the **access control and identity layer**. It:
//! - Authenticates who is making the request
//! - Determines what capabilities they have access to
//! - Scopes queries to the appropriate domain
//! - Prevents unauthorized access to higher-tier operations
//!
//! A request without a Zanpakuto can only use basic Asauchi capabilities.
//! A named Zanpakuto unlocks Shikai (queries). A mastered one unlocks Bankai (solves).

pub mod nlp;

pub use nlp::{NlpContext, NlpEngine, QueryType};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::wheel::Domain;

/// Errors from access control.
#[derive(Error, Debug)]
pub enum ZanpakutoError {
    #[error("unauthenticated: no identity provided")]
    Unauthenticated,

    #[error("unauthorized: {action} requires {required:?}, you have {actual:?}")]
    Unauthorized {
        action: String,
        required: Vec<Capability>,
        actual: Vec<Capability>,
    },

    #[error("invalid identity: {0}")]
    InvalidIdentity(String),

    #[error("session expired")]
    SessionExpired,
}

/// Capabilities that can be granted to an identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Capability {
    /// Basic public access (Asauchi level)
    Public,
    /// Access to primitive formulas
    PrimitiveAccess,
    /// Access to entities and their properties
    EntityAccess,
    /// Can traverse the wheel graph
    Traversal,
    /// Can compose formulas
    Composition,
    /// Can execute full chains
    ChainExecution,
    /// Admin — full access
    Admin,
}

/// Access tier — how "released" the system is for this request.
///
/// Maps to the 4 Bleach layers along the understanding axis:
///
/// | Tier       | Level | State    | Capabilities                          |
/// |------------|-------|----------|---------------------------------------|
/// | `Asauchi`  | 0     | Unknown  | Public access only                    |
/// | `Zanpakuto`| 3     | Aware    | + Named identity, primitive formulas  |
/// | `Shikai`   | 6     | Learning | + Traversal, entity access            |
/// | `Bankai`   | 12    | Known    | + Composition, chains, admin          |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessTier {
    /// No identity — public only (Level 0: Unknown)
    Asauchi,
    /// Named identity — aware, basic access (Level 3: Aware)
    Zanpakuto,
    /// First release — active learning, traversal (Level 6: Learning)
    Shikai,
    /// Full release — complete mastery (Level 12: Known)
    Bankai,
}

impl AccessTier {
    /// Map this access tier to the corresponding stage of mastery.
    ///
    /// | Stage         | Tiers                 | Levels |
    /// |---------------|-----------------------|--------|
    /// | `Seeker`      | Asauchi, Zanpakuto    | 0–3    |
    /// | `Practitioner`| Shikai                | 4–6    |
    /// | `Sovereign`   | Bankai                | 7–12   |
    pub fn stage(self) -> Stage {
        match self {
            AccessTier::Asauchi | AccessTier::Zanpakuto => Stage::Seeker,
            AccessTier::Shikai => Stage::Practitioner,
            AccessTier::Bankai => Stage::Sovereign,
        }
    }

    /// Level number (0, 3, 6, or 12).
    pub fn level(self) -> u8 {
        match self {
            AccessTier::Asauchi => 0,
            AccessTier::Zanpakuto => 3,
            AccessTier::Shikai => 6,
            AccessTier::Bankai => 12,
        }
    }

    /// Human-readable state name.
    pub fn state_name(self) -> &'static str {
        match self {
            AccessTier::Asauchi => "Unknown",
            AccessTier::Zanpakuto => "Aware",
            AccessTier::Shikai => "Learning",
            AccessTier::Bankai => "Known",
        }
    }
}

/// Stage of mastery — a three-stage path from discovery to mastery.
///
/// Maps the 4-layer Bleach tier system onto a simpler 3-stage progression:
///
/// | Stage         | Purpose                              | AccessTiers      |
/// |---------------|--------------------------------------|------------------|
/// | `Seeker`      | Discovery — exploring, learning      | Asauchi + Zanpakuto |
/// | `Practitioner`| Application — actively using tools   | Shikai           |
/// | `Sovereign`   | Mastery — full command, extending    | Bankai           |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stage {
    /// Discovery stage — exploring the system, learning basics.
    /// Corresponds to Asauchi (public) and Zanpakuto (named identity).
    Seeker,
    /// Application stage — actively using formulas, composing queries.
    /// Corresponds to Shikai.
    Practitioner,
    /// Mastery stage — full command, extending the system.
    /// Corresponds to Bankai.
    Sovereign,
}

impl Stage {
    /// All stages in progression order.
    pub const ALL: [Stage; 3] = [Stage::Seeker, Stage::Practitioner, Stage::Sovereign];

    /// Derive the stage from a numeric level (0–12).
    pub fn from_level(level: u8) -> Self {
        match level {
            0..=3 => Stage::Seeker,
            4..=6 => Stage::Practitioner,
            _ => Stage::Sovereign, // 7–12
        }
    }

    /// Get the stage one step forward in progression.
    pub fn next(self) -> Option<Stage> {
        match self {
            Stage::Seeker => Some(Stage::Practitioner),
            Stage::Practitioner => Some(Stage::Sovereign),
            Stage::Sovereign => None,
        }
    }

    /// Get the stage one step backward in progression.
    pub fn prev(self) -> Option<Stage> {
        match self {
            Stage::Seeker => None,
            Stage::Practitioner => Some(Stage::Seeker),
            Stage::Sovereign => Some(Stage::Practitioner),
        }
    }

    /// Human-readable description of this stage's purpose.
    pub fn description(self) -> &'static str {
        match self {
            Stage::Seeker => "Discovery — exploring the system, building foundational knowledge",
            Stage::Practitioner => {
                "Application — actively using tools, composing queries, cross-domain reasoning"
            }
            Stage::Sovereign => {
                "Mastery — full command, extending the system, creating new formulas"
            }
        }
    }
}

impl std::fmt::Display for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stage::Seeker => write!(f, "Seeker"),
            Stage::Practitioner => write!(f, "Practitioner"),
            Stage::Sovereign => write!(f, "Sovereign"),
        }
    }
}

/// An identity that has been authenticated by the Zanpakuto.
#[derive(Debug, Clone, Serialize)]
pub struct Identity {
    /// The name of this identity (the "Zanpakuto name").
    pub name: String,
    /// The access tier granted.
    pub tier: AccessTier,
    /// The capabilities this identity has.
    pub capabilities: Vec<Capability>,
    /// Domains this identity is restricted to (empty = all domains).
    pub scope: Vec<Domain>,
    /// Session token.
    pub session: String,
}

/// The Zanpakuto — access controller.
///
/// Authenticates identities and grants capabilities based on the access tier.
#[derive(Debug, Clone)]
pub struct Zanpakuto {
    /// Registered identities.
    identities: Vec<Identity>,
    /// Default capabilities for unauthenticated users.
    default_capabilities: Vec<Capability>,
}

impl Default for Zanpakuto {
    fn default() -> Self {
        Self::new()
    }
}

impl Zanpakuto {
    /// Create a new Zanpakuto access controller.
    pub fn new() -> Self {
        Zanpakuto {
            identities: Vec::new(),
            default_capabilities: vec![Capability::Public],
        }
    }

    /// Register a new identity.
    pub fn register(&mut self, name: &str, tier: AccessTier) -> Identity {
        let capabilities = match tier {
            AccessTier::Asauchi => vec![Capability::Public],
            AccessTier::Zanpakuto => vec![Capability::Public, Capability::PrimitiveAccess],
            AccessTier::Shikai => vec![
                Capability::Public,
                Capability::PrimitiveAccess,
                Capability::EntityAccess,
                Capability::Traversal,
            ],
            AccessTier::Bankai => vec![
                Capability::Public,
                Capability::PrimitiveAccess,
                Capability::EntityAccess,
                Capability::Traversal,
                Capability::Composition,
                Capability::ChainExecution,
                Capability::Admin,
            ],
        };

        let identity = Identity {
            name: name.to_string(),
            tier,
            capabilities,
            scope: Vec::new(),
            session: format!("session_{}", name),
        };

        self.identities.push(identity.clone());
        identity
    }

    /// Authenticate a session token.
    pub fn authenticate(&self, session: &str) -> Result<&Identity, ZanpakutoError> {
        self.identities
            .iter()
            .find(|id| id.session == session)
            .ok_or(ZanpakutoError::Unauthenticated)
    }

    /// Authorize a specific capability.
    pub fn authorize(
        &self,
        identity: &Identity,
        required: Capability,
    ) -> Result<(), ZanpakutoError> {
        if identity.capabilities.contains(&required)
            || identity.capabilities.contains(&Capability::Admin)
        {
            Ok(())
        } else {
            Err(ZanpakutoError::Unauthorized {
                action: format!("requires {:?}", required),
                required: vec![required],
                actual: identity.capabilities.clone(),
            })
        }
    }

    /// Get the default capabilities for unauthenticated access.
    pub fn default_access(&self) -> &[Capability] {
        &self.default_capabilities
    }

    /// Check if a domain is within an identity's scope.
    pub fn in_scope(&self, identity: &Identity, domain: Domain) -> bool {
        identity.scope.is_empty() || identity.scope.contains(&domain)
    }
}

/// Guest identity for unauthenticated access.
pub fn guest_identity() -> Identity {
    Identity {
        name: "guest".to_string(),
        tier: AccessTier::Asauchi,
        capabilities: vec![Capability::Public],
        scope: Vec::new(),
        session: "guest_session".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zanpakuto_create() {
        let z = Zanpakuto::new();
        assert!(z.identities.is_empty());
    }

    #[test]
    fn test_register_zanpakuto() {
        let mut z = Zanpakuto::new();
        let id = z.register("user1", AccessTier::Zanpakuto);
        assert_eq!(id.name, "user1");
        assert_eq!(id.tier, AccessTier::Zanpakuto);
        assert!(id.capabilities.contains(&Capability::PrimitiveAccess));
        assert!(!id.capabilities.contains(&Capability::Traversal));
    }

    #[test]
    fn test_register_shikai() {
        let mut z = Zanpakuto::new();
        let id = z.register("user1", AccessTier::Shikai);
        assert_eq!(id.name, "user1");
        assert_eq!(id.tier, AccessTier::Shikai);
        assert!(id.capabilities.contains(&Capability::PrimitiveAccess));
        assert!(!id.capabilities.contains(&Capability::ChainExecution));
    }

    #[test]
    fn test_register_bankai() {
        let mut z = Zanpakuto::new();
        let id = z.register("admin", AccessTier::Bankai);
        assert!(id.capabilities.contains(&Capability::ChainExecution));
        assert!(id.capabilities.contains(&Capability::Admin));
    }

    #[test]
    fn test_authenticate_valid() {
        let mut z = Zanpakuto::new();
        let id = z.register("user", AccessTier::Shikai);
        let auth = z.authenticate(&id.session).unwrap();
        assert_eq!(auth.name, "user");
    }

    #[test]
    fn test_authenticate_invalid() {
        let z = Zanpakuto::new();
        assert!(z.authenticate("bad_session").is_err());
    }

    #[test]
    fn test_authorize_public_cant_compose() {
        let mut z = Zanpakuto::new();
        let id = z.register("user", AccessTier::Asauchi);
        assert!(z.authorize(&id, Capability::Composition).is_err());
    }

    #[test]
    fn test_authorize_zanpakuto_can_atomic() {
        let mut z = Zanpakuto::new();
        let id = z.register("user", AccessTier::Zanpakuto);
        assert!(z.authorize(&id, Capability::PrimitiveAccess).is_ok());
        assert!(z.authorize(&id, Capability::Composition).is_err());
    }

    #[test]
    fn test_authorize_bankai_can_do_anything() {
        let mut z = Zanpakuto::new();
        let id = z.register("admin", AccessTier::Bankai);
        assert!(z.authorize(&id, Capability::Composition).is_ok());
        assert!(z.authorize(&id, Capability::Admin).is_ok());
    }

    // ─── Stage progression (Phase 5) ───────────────────────────────

    #[test]
    fn test_stage_from_access_tier() {
        assert_eq!(AccessTier::Asauchi.stage(), Stage::Seeker);
        assert_eq!(AccessTier::Zanpakuto.stage(), Stage::Seeker);
        assert_eq!(AccessTier::Shikai.stage(), Stage::Practitioner);
        assert_eq!(AccessTier::Bankai.stage(), Stage::Sovereign);
    }

    #[test]
    fn test_stage_from_level() {
        assert_eq!(Stage::from_level(0), Stage::Seeker);
        assert_eq!(Stage::from_level(3), Stage::Seeker);
        assert_eq!(Stage::from_level(4), Stage::Practitioner);
        assert_eq!(Stage::from_level(6), Stage::Practitioner);
        assert_eq!(Stage::from_level(7), Stage::Sovereign);
        assert_eq!(Stage::from_level(12), Stage::Sovereign);
    }

    #[test]
    fn test_stage_progression() {
        assert_eq!(Stage::Seeker.next(), Some(Stage::Practitioner));
        assert_eq!(Stage::Practitioner.next(), Some(Stage::Sovereign));
        assert_eq!(Stage::Sovereign.next(), None);
        assert_eq!(Stage::Practitioner.prev(), Some(Stage::Seeker));
        assert_eq!(Stage::Seeker.prev(), None);
    }

    #[test]
    fn test_stage_description() {
        assert!(Stage::Seeker.description().contains("Discovery"));
        assert!(Stage::Practitioner.description().contains("Application"));
        assert!(Stage::Sovereign.description().contains("Mastery"));
    }

    #[test]
    fn test_stage_all() {
        assert_eq!(Stage::ALL.len(), 3);
        assert_eq!(Stage::ALL[0], Stage::Seeker);
        assert_eq!(Stage::ALL[1], Stage::Practitioner);
        assert_eq!(Stage::ALL[2], Stage::Sovereign);
    }

    #[test]
    fn test_access_tier_level() {
        assert_eq!(AccessTier::Asauchi.level(), 0);
        assert_eq!(AccessTier::Zanpakuto.level(), 3);
        assert_eq!(AccessTier::Shikai.level(), 6);
        assert_eq!(AccessTier::Bankai.level(), 12);
    }

    #[test]
    fn test_access_tier_state_name() {
        assert_eq!(AccessTier::Asauchi.state_name(), "Unknown");
        assert_eq!(AccessTier::Zanpakuto.state_name(), "Aware");
        assert_eq!(AccessTier::Shikai.state_name(), "Learning");
        assert_eq!(AccessTier::Bankai.state_name(), "Known");
    }
}
