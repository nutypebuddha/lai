/// Pure function: Format CLI output for display.
pub fn format_cli_output(label: &str, value: &str) -> String {
    format!("{label}: {value}")
}

/// Pure function: Parse key=value arguments from CLI.
pub fn parse_key_value_arguments(arguments: &str) -> Vec<(String, String)> {
    arguments
        .split_whitespace()
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?.to_string();
            let value = parts.next()?.to_string();
            Some((key, value))
        })
        .collect()
}

/// Pure function: Validate CLI subcommand name.
pub fn validate_subcommand_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_cli_output_basic() {
        assert_eq!(format_cli_output("version", "1.0"), "version: 1.0");
    }

    #[test]
    fn parse_key_value_arguments_basic() {
        let args = parse_key_value_arguments("key1=value1 key2=value2");
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], ("key1".to_string(), "value1".to_string()));
    }

    #[test]
    fn validate_subcommand_name_basic() {
        assert!(validate_subcommand_name("solve"));
        assert!(validate_subcommand_name("validate-formula"));
        assert!(!validate_subcommand_name(""));
        assert!(!validate_subcommand_name("has space"));
    }
}
