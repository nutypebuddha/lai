/// Pure function: Normalize query text for NLP processing.
/// Strips non-alphanumeric, lowercases, collapses whitespace.
pub fn normalize_query_text(input: &str) -> String {
    let cleaned: String = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();
    cleaned.split_whitespace().collect::<Vec<&str>>().join(" ")
}

/// Pure function: Extract keywords from normalized query.
pub fn extract_keywords(normalized_query: &str) -> Vec<String> {
    normalized_query
        .split_whitespace()
        .filter(|word| word.len() > 2)
        .map(|word| word.to_string())
        .collect()
}

/// Pure function: Compute query intent score based on keyword presence.
pub fn compute_intent_score(keywords: &[String], intent_markers: &[&str]) -> f64 {
    if keywords.is_empty() || intent_markers.is_empty() {
        return 0.0;
    }
    let matches = keywords
        .iter()
        .filter(|keyword| intent_markers.iter().any(|marker| keyword.contains(marker)))
        .count();
    matches as f64 / keywords.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_query_text_basic() {
        assert_eq!(normalize_query_text("Hello World!"), "hello world");
        assert_eq!(normalize_query_text("  SPACED  "), "spaced");
        assert_eq!(normalize_query_text("hello   world"), "hello world");
    }

    #[test]
    fn extract_keywords_basic() {
        assert_eq!(
            extract_keywords("what is the meaning"),
            vec!["what", "the", "meaning"]
        );
        assert!(extract_keywords("a b").is_empty());
    }

    #[test]
    fn compute_intent_score_basic() {
        let keywords = vec!["what".to_string(), "meaning".to_string(), "of".to_string()];
        let markers = ["what", "how"];
        assert!((compute_intent_score(&keywords, &markers) - 1.0 / 3.0).abs() < f64::EPSILON);
    }
}
