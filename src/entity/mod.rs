/// Pure function: Validate an entity ID string.
pub fn validate_entity_id(entity_id: &str) -> bool {
    !entity_id.is_empty() && entity_id.len() <= 128
}

/// Pure function: Compute entity hash for deduplication.
pub fn compute_entity_hash(entity_id: &str) -> u64 {
    let mut hash: u64 = 0;
    for byte in entity_id.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    hash
}

/// Pure function: Check if two entity IDs refer to the same entity.
pub fn is_same_entity(left_id: &str, right_id: &str) -> bool {
    left_id.eq_ignore_ascii_case(right_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_entity_id_basic() {
        assert!(validate_entity_id("surya"));
        assert!(validate_entity_id("graha_mangala"));
        assert!(!validate_entity_id(""));
    }

    #[test]
    fn compute_entity_hash_basic() {
        let hash_a = compute_entity_hash("surya");
        let hash_b = compute_entity_hash("surya");
        let hash_c = compute_entity_hash("chandra");
        assert_eq!(hash_a, hash_b);
        assert_ne!(hash_a, hash_c);
    }

    #[test]
    fn is_same_entity_basic() {
        assert!(is_same_entity("surya", "surya"));
        assert!(is_same_entity("Surya", "surya"));
        assert!(!is_same_entity("surya", "chandra"));
    }
}
