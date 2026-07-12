/// Pure function: Validate MCP tool name.
pub fn validate_tool_name(tool_name: &str) -> bool {
    !tool_name.is_empty()
        && tool_name.len() <= 64
        && tool_name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}

/// Pure function: Compute MCP request hash for caching.
pub fn compute_request_hash(method: &str, params: &str) -> u64 {
    let mut hash: u64 = 0;
    for byte in method.bytes().chain(params.bytes()) {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    hash
}

/// Pure function: Check if MCP response needs compaction.
pub fn needs_response_compaction(response_size: usize, max_size: usize) -> bool {
    response_size > max_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_tool_name_basic() {
        assert!(validate_tool_name("solve"));
        assert!(validate_tool_name("validate_formula"));
        assert!(!validate_tool_name(""));
        assert!(!validate_tool_name("has space"));
    }

    #[test]
    fn compute_request_hash_basic() {
        let hash_a = compute_request_hash("tools/call", "{}");
        let hash_b = compute_request_hash("tools/call", "{}");
        let hash_c = compute_request_hash("tools/call", "{\"query\":\"test\"}");
        assert_eq!(hash_a, hash_b);
        assert_ne!(hash_a, hash_c);
    }

    #[test]
    fn needs_response_compaction_basic() {
        assert!(needs_response_compaction(1000, 500));
        assert!(!needs_response_compaction(100, 500));
    }
}
