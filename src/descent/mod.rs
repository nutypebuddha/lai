/// Pure function: Lowercase a string. No side effects.
pub fn lowercase_string(input: &str) -> String {
    input.to_lowercase()
}

/// Pure function: Tokenize input into descent layers.
pub fn tokenize_descent(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .map(|token| token.to_lowercase())
        .collect()
}

/// Pure function: Normalize whitespace in input string.
pub fn normalize_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<&str>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercase_string_basic() {
        assert_eq!(lowercase_string("HELLO"), "hello");
        assert_eq!(lowercase_string("Hello World"), "hello world");
        assert_eq!(lowercase_string(""), "");
    }

    #[test]
    fn tokenize_descent_basic() {
        assert_eq!(tokenize_descent("HELLO WORLD"), vec!["hello", "world"]);
        assert_eq!(tokenize_descent("  spaced  out  "), vec!["spaced", "out"]);
    }

    #[test]
    fn normalize_whitespace_basic() {
        assert_eq!(
            normalize_whitespace("  multiple   spaces  "),
            "multiple spaces"
        );
        assert_eq!(normalize_whitespace("tab\there"), "tab here");
    }
}
