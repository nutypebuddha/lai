//! # Asauchi — the public-facing layer
//!
//! Asauchi is the base blade — the form everyone knows about. It's the public face of Athena:
//! what people see when they first encounter the system. It provides version info, system
//! identity, and the public API that routes requests into the deeper layers.
//!
//! In the Bleach analogy:
//! - **Asauchi** = the nameless sword all Shinigami carry = Athena's public interface
//! - Everyone knows it exists. It's the entry point.
//! - It has limited power on its own — real power comes from naming it (Zanpakuto).

use crate::gates::confidence::ConfidenceGate;
use crate::gates::fact::FactGate;
use crate::gates::formal::FormalGate;
use crate::gates::logic::LogicGate;
use crate::gates::math::MathGate;
use crate::gates::{Gate, GateResult};

use serde::Serialize;

/// System identity — the public face of Athena.
#[derive(Debug, Clone, Serialize)]
pub struct Asauchi {
    /// System name
    pub name: &'static str,
    /// Version
    pub version: &'static str,
    /// Build timestamp
    pub build_time: &'static str,
    /// Tagline
    pub tagline: &'static str,
    /// Layer status
    pub layer: &'static str,
    /// Public capability flags
    pub capabilities: Vec<&'static str>,
}

impl Default for Asauchi {
    fn default() -> Self {
        Self::new()
    }
}

impl Asauchi {
    /// Create a new Asauchi instance.
    pub fn new() -> Self {
        Asauchi {
            name: "Athena",
            version: crate::VERSION,
            build_time: crate::BUILD_TIME,
            tagline: "Relational intelligence — formulas, not facts",
            layer: "asauchi",
            capabilities: vec![
                "vedic_wheel:9_grahas",
                "rashis:12_sidereal_signs",
                "nakshatras:27_lunar_mansions",
                "tattvas:5_elements_with_akasha",
                "gunas:3_primordial_qualities",
                "purusharthas:4_life_goals",
                "primitive_formulas:22_gates",
                "vedic_entity_system",
                "validation_gates:math_logic_confidence_formal",
                "cross_domain_traversal",
                "mcp_server",
                "cli_interface",
            ],
        }
    }

    /// Ping the system — returns basic identity.
    pub fn ping(&self) -> AsauchiResponse {
        AsauchiResponse {
            name: self.name,
            version: self.version,
            layer: self.layer,
            message: "Athena is ready. Speak its name to awaken the Zanpakuto.",
        }
    }

    /// Get system info.
    pub fn info(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "version": self.version,
            "build_time": self.build_time,
            "tagline": self.tagline,
            "layer": self.layer,
            "capabilities": self.capabilities,
            "message": "Asauchi: the known blade. Name it to unlock your Zanpakuto.",
        })
    }

    /// Public validation — runs basic gates without authentication.
    pub fn public_validate(&self, text: &str, gate: &str) -> GateResult {
        let gates: Vec<Box<dyn Gate>> = match gate {
            "math" => vec![Box::new(MathGate::new())],
            "logic" => vec![Box::new(LogicGate::new())],
            "formal" => vec![Box::new(FormalGate::new())],
            "fact" => vec![Box::new(FactGate::new(None))],
            "confidence" => vec![Box::new(ConfidenceGate::new())],
            "all" => vec![
                Box::new(MathGate::new()),
                Box::new(LogicGate::new()),
                Box::new(FormalGate::new()),
                Box::new(ConfidenceGate::new()),
            ],
            _ => vec![Box::new(MathGate::new())],
        };
        GateResult::run(gates, text)
    }
}

/// A response from the Asauchi layer.
#[derive(Debug, Serialize)]
pub struct AsauchiResponse {
    pub name: &'static str,
    pub version: &'static str,
    pub layer: &'static str,
    pub message: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asauchi_creation() {
        let a = Asauchi::new();
        assert_eq!(a.name, "Athena");
        assert_eq!(a.layer, "asauchi");
    }

    #[test]
    fn test_asauchi_ping() {
        let a = Asauchi::new();
        let p = a.ping();
        assert_eq!(p.name, "Athena");
        assert!(p.message.contains("Zanpakuto"));
    }

    #[test]
    fn test_public_validation() {
        let a = Asauchi::new();
        let result = a.public_validate("2 + 2", "math");
        assert!(!result.gates.is_empty());
    }

    #[test]
    fn test_capabilities() {
        let a = Asauchi::new();
        assert!(a.capabilities.len() >= 5);
    }
}
