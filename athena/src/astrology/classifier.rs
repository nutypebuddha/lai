//! # Change Sorter — Token Classification Engine
//!
//! The Change Sorter is a multi-stage classifier that takes a text token
//! and produces a weighted AtomClassification across all 7 astrology axes.
//!
//! Every token is classified against ALL axes simultaneously — not picking
//! one domain but assigning weighted scores to every sign, element, modality,
//! planetary ruler, house, aspect, and polarity.
//!
//! This is currently a stub. The full implementation will use keyword maps,
//! semantic embeddings, and astrological lookup tables.

use std::collections::HashMap;

use super::AtomClassification;

/// The Change Sorter — classifies tokens across all 7 astrology axes.
#[derive(Debug, Clone)]
pub struct ChangeSorter {
    /// Keyword → sign activation map (build at initialization)
    sign_keywords: HashMap<String, Vec<(usize, f64)>>,
    /// Keyword → element activation map
    element_keywords: HashMap<String, Vec<(usize, f64)>>,
    /// Keyword → modality activation map
    modality_keywords: HashMap<String, Vec<(usize, f64)>>,
    /// Keyword → ruler activation map
    ruler_keywords: HashMap<String, Vec<(usize, f64)>>,
}

impl Default for ChangeSorter {
    fn default() -> Self {
        Self::new()
    }
}

impl ChangeSorter {
    /// Create a new ChangeSorter with default keyword mappings.
    pub fn new() -> Self {
        let mut sorter = ChangeSorter {
            sign_keywords: HashMap::new(),
            element_keywords: HashMap::new(),
            modality_keywords: HashMap::new(),
            ruler_keywords: HashMap::new(),
        };
        sorter.init_default_mappings();
        sorter
    }

    /// Initialize the default keyword→astrology mappings.
    fn init_default_mappings(&mut self) {
        // Sign keywords
        let sign_mappings = [
            // Aries — Math & Logic
            (0, "add", 0.8),
            (0, "subtract", 0.8),
            (0, "multiply", 0.8),
            (0, "divide", 0.8),
            (0, "math", 0.9),
            (0, "logic", 0.8),
            (0, "number", 0.7),
            (0, "calculate", 0.7),
            (0, "prove", 0.6),
            (0, "theorem", 0.7),
            (0, "axiom", 0.6),
            // Taurus — Physics & Chemistry
            (1, "force", 0.8),
            (1, "mass", 0.8),
            (1, "energy", 0.8),
            (1, "velocity", 0.7),
            (1, "acceleration", 0.8),
            (1, "physics", 0.9),
            (1, "chemistry", 0.8),
            (1, "atom", 0.6),
            (1, "molecule", 0.6),
            (1, "gravity", 0.7),
            (1, "motion", 0.7),
            // Gemini — Astronomy & Cosmology
            (2, "star", 0.8),
            (2, "planet", 0.8),
            (2, "galaxy", 0.8),
            (2, "cosmos", 0.8),
            (2, "astronomy", 0.9),
            (2, "orbit", 0.7),
            (2, "space", 0.7),
            (2, "universe", 0.7),
            (2, "light", 0.6),
            (2, "gravity", 0.5),
            // Cancer — Earth & Environment
            (3, "earth", 0.8),
            (3, "water", 0.7),
            (3, "air", 0.6),
            (3, "climate", 0.8),
            (3, "weather", 0.7),
            (3, "ocean", 0.7),
            (3, "environment", 0.8),
            (3, "ecosystem", 0.7),
            (3, "nature", 0.6),
            // Leo — Biology & Medicine
            (4, "cell", 0.7),
            (4, "dna", 0.8),
            (4, "gene", 0.7),
            (4, "protein", 0.7),
            (4, "biology", 0.9),
            (4, "medicine", 0.8),
            (4, "organ", 0.6),
            (4, "tissue", 0.6),
            (4, "evolution", 0.7),
            // Virgo — Economics & Finance
            (5, "price", 0.7),
            (5, "cost", 0.7),
            (5, "market", 0.8),
            (5, "economy", 0.9),
            (5, "finance", 0.8),
            (5, "money", 0.7),
            (5, "trade", 0.7),
            (5, "budget", 0.6),
            (5, "tax", 0.6),
            // Libra — Engineering & Tech
            (6, "engineer", 0.8),
            (6, "design", 0.7),
            (6, "machine", 0.7),
            (6, "circuit", 0.7),
            (6, "bridge", 0.6),
            (6, "structure", 0.7),
            (6, "technology", 0.8),
            (6, "system", 0.7),
            (6, "mechanics", 0.7),
            // Scorpio — Computer Science & AI
            (7, "computer", 0.8),
            (7, "algorithm", 0.8),
            (7, "data", 0.7),
            (7, "code", 0.7),
            (7, "program", 0.7),
            (7, "software", 0.7),
            (7, "ai", 0.8),
            (7, "intelligence", 0.7),
            (7, "neural", 0.6),
            // Sagittarius — History & Anthropology
            (8, "history", 0.9),
            (8, "culture", 0.7),
            (8, "ancient", 0.7),
            (8, "civilization", 0.7),
            (8, "anthropology", 0.8),
            (8, "society", 0.6),
            (8, "tradition", 0.6),
            (8, "origin", 0.6),
            // Capricorn — Language & Linguistics
            (9, "word", 0.7),
            (9, "language", 0.9),
            (9, "grammar", 0.8),
            (9, "linguistics", 0.8),
            (9, "syntax", 0.7),
            (9, "semantics", 0.7),
            (9, "speech", 0.6),
            (9, "text", 0.6),
            // Aquarius — Philosophy & Ethics
            (10, "truth", 0.7),
            (10, "ethics", 0.8),
            (10, "philosophy", 0.9),
            (10, "moral", 0.7),
            (10, "justice", 0.7),
            (10, "virtue", 0.6),
            (10, "reason", 0.7),
            (10, "knowledge", 0.7),
            // Pisces — Psychology & Neuroscience
            (11, "mind", 0.7),
            (11, "brain", 0.8),
            (11, "psychology", 0.9),
            (11, "neuron", 0.7),
            (11, "consciousness", 0.7),
            (11, "emotion", 0.7),
            (11, "behavior", 0.6),
            (11, "cognition", 0.7),
        ];

        for (sign_idx, keyword, weight) in &sign_mappings {
            self.sign_keywords
                .entry(keyword.to_string())
                .or_default()
                .push((*sign_idx, *weight));
        }

        // Element keywords
        let element_mappings = [
            (0, "action", 0.7),
            (0, "energy", 0.6),
            (0, "force", 0.6),
            (0, "heat", 0.7),
            (1, "structure", 0.7),
            (1, "solid", 0.6),
            (1, "material", 0.7),
            (1, "ground", 0.6),
            (2, "thought", 0.6),
            (2, "idea", 0.6),
            (2, "communication", 0.7),
            (2, "information", 0.6),
            (3, "feeling", 0.7),
            (3, "emotion", 0.7),
            (3, "flow", 0.6),
            (3, "intuition", 0.6),
        ];

        for (elem_idx, keyword, weight) in &element_mappings {
            self.element_keywords
                .entry(keyword.to_string())
                .or_default()
                .push((*elem_idx, *weight));
        }

        // Modality keywords
        let modality_mappings = [
            (0, "initiate", 0.8),
            (0, "start", 0.7),
            (0, "lead", 0.7),
            (0, "begin", 0.6),
            (1, "stable", 0.7),
            (1, "endure", 0.7),
            (1, "persist", 0.7),
            (1, "steady", 0.6),
            (2, "change", 0.7),
            (2, "adapt", 0.7),
            (2, "flexible", 0.7),
            (2, "transform", 0.6),
        ];

        for (mod_idx, keyword, weight) in &modality_mappings {
            self.modality_keywords
                .entry(keyword.to_string())
                .or_default()
                .push((*mod_idx, *weight));
        }

        // Ruler keywords
        let ruler_mappings = [
            (0, "sun", 0.9),
            (0, "light", 0.6),
            (0, "shine", 0.5),
            (1, "moon", 0.9),
            (1, "lunar", 0.7),
            (1, "night", 0.5),
            (2, "mercury", 0.9),
            (2, "message", 0.5),
            (2, "communicate", 0.6),
            (3, "venus", 0.9),
            (3, "beauty", 0.6),
            (3, "love", 0.5),
            (4, "mars", 0.9),
            (4, "war", 0.6),
            (4, "battle", 0.5),
            (5, "jupiter", 0.9),
            (5, "luck", 0.5),
            (5, "expand", 0.6),
            (6, "saturn", 0.9),
            (6, "time", 0.6),
            (6, "structure", 0.6),
        ];

        for (ruler_idx, keyword, weight) in &ruler_mappings {
            self.ruler_keywords
                .entry(keyword.to_string())
                .or_default()
                .push((*ruler_idx, *weight));
        }
    }

    /// Classify a single token across all 7 astrology axes.
    ///
    /// Returns an AtomClassification with weighted activation values
    /// for each axis based on keyword matches.
    pub fn classify_token(&self, token: &str) -> AtomClassification {
        let lower = token.to_lowercase();
        let mut result = AtomClassification::new();

        // Sign classification
        if let Some(activations) = self.sign_keywords.get(&lower) {
            for &(idx, weight) in activations {
                result.signs[idx] = result.signs[idx].max(weight);
            }
        }

        // Element classification
        if let Some(activations) = self.element_keywords.get(&lower) {
            for &(idx, weight) in activations {
                result.elements[idx] = result.elements[idx].max(weight);
            }
        }

        // Modality classification
        if let Some(activations) = self.modality_keywords.get(&lower) {
            for &(idx, weight) in activations {
                result.modalities[idx] = result.modalities[idx].max(weight);
            }
        }

        // Ruler classification
        if let Some(activations) = self.ruler_keywords.get(&lower) {
            for &(idx, weight) in activations {
                result.rulers[idx] = result.rulers[idx].max(weight);
            }
        }

        result
    }

    /// Classify a multi-token query by summing individual token classifications.
    pub fn classify_query(&self, query: &str) -> AtomClassification {
        let mut result = AtomClassification::new();
        let tokens: Vec<&str> = query
            .split(|c: char| !c.is_alphanumeric())
            .filter(|t| !t.is_empty())
            .collect();

        for token in &tokens {
            let token_class = self.classify_token(token);
            result = result.merge_sum(&token_class);
        }

        result
    }

    /// Check if the sorter is uninitialized (no mappings).
    pub fn is_empty(&self) -> bool {
        self.sign_keywords.is_empty()
            && self.element_keywords.is_empty()
            && self.modality_keywords.is_empty()
            && self.ruler_keywords.is_empty()
    }

    /// Number of keyword mappings across all axes.
    pub fn mapping_count(&self) -> usize {
        self.sign_keywords.len()
            + self.element_keywords.len()
            + self.modality_keywords.len()
            + self.ruler_keywords.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_math_token() {
        let sorter = ChangeSorter::new();
        let result = sorter.classify_token("math");
        assert!(result.signs[0] > 0.5, "math should activate Aries (sign 0)"); // Aries
    }

    #[test]
    fn test_classify_physics_token() {
        let sorter = ChangeSorter::new();
        let result = sorter.classify_token("force");
        assert!(
            result.signs[1] > 0.5,
            "force should activate Taurus (sign 1)"
        ); // Taurus
    }

    #[test]
    fn test_classify_query() {
        let sorter = ChangeSorter::new();
        let result = sorter.classify_query("calculate kinetic energy");
        // Should activate multiple signs
        assert!(result.signs.iter().any(|&v| v > 0.0));
    }

    #[test]
    fn test_unknown_token() {
        let sorter = ChangeSorter::new();
        let result = sorter.classify_token("xyznonexistent");
        assert_eq!(result, AtomClassification::new());
    }

    #[test]
    fn test_case_insensitive() {
        let sorter = ChangeSorter::new();
        let a = sorter.classify_token("MATH");
        let b = sorter.classify_token("math");
        assert_eq!(a, b);
    }

    #[test]
    fn test_is_empty_after_new() {
        let sorter = ChangeSorter::new();
        assert!(!sorter.is_empty());
    }

    #[test]
    fn test_element_classification() {
        let sorter = ChangeSorter::new();
        let result = sorter.classify_token("action");
        assert!(result.elements[0] > 0.5); // Fire
    }

    #[test]
    fn test_modality_classification() {
        let sorter = ChangeSorter::new();
        let result = sorter.classify_token("initiate");
        assert!(result.modalities[0] > 0.5); // Cardinal
    }

    #[test]
    fn test_ruler_classification() {
        let sorter = ChangeSorter::new();
        let result = sorter.classify_token("mars");
        assert!(result.rulers[4] > 0.5); // Mars
    }

    #[test]
    fn test_mapping_count() {
        let sorter = ChangeSorter::new();
        assert!(sorter.mapping_count() > 20);
    }
}
