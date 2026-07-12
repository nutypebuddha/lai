/// Pure function: NAND gate. Universal gate — all logic derives from this.
pub fn nand_gate(left_input: bool, right_input: bool) -> bool {
    !(left_input && right_input)
}

/// Pure function: NOT gate. Derived from NAND with both inputs tied.
pub fn not_gate(input: bool) -> bool {
    nand_gate(input, input)
}

/// Pure function: AND gate. Derived from NAND + NOT.
pub fn and_gate(left_input: bool, right_input: bool) -> bool {
    not_gate(nand_gate(left_input, right_input))
}

/// Pure function: OR gate. Derived from NAND of NOTs.
pub fn or_gate(left_input: bool, right_input: bool) -> bool {
    nand_gate(not_gate(left_input), not_gate(right_input))
}

/// Pure function: XOR gate. Derived from NAND + OR.
pub fn xor_gate(left_input: bool, right_input: bool) -> bool {
    and_gate(
        nand_gate(left_input, right_input),
        or_gate(left_input, right_input),
    )
}

/// Pure function: XNOR gate. Inverse of XOR.
pub fn xnor_gate(left_input: bool, right_input: bool) -> bool {
    not_gate(xor_gate(left_input, right_input))
}

/// Pure function: IMPLICATION gate. left_input implies right_input.
pub fn implication_gate(left_input: bool, right_input: bool) -> bool {
    or_gate(not_gate(left_input), right_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nand_gate_table() {
        assert!(!nand_gate(true, true));
        assert!(nand_gate(true, false));
        assert!(nand_gate(false, true));
        assert!(nand_gate(false, false));
    }

    #[test]
    fn not_gate_table() {
        assert!(!not_gate(true));
        assert!(not_gate(false));
    }

    #[test]
    fn and_gate_table() {
        assert!(and_gate(true, true));
        assert!(!and_gate(true, false));
        assert!(!and_gate(false, true));
        assert!(!and_gate(false, false));
    }

    #[test]
    fn or_gate_table() {
        assert!(or_gate(true, true));
        assert!(or_gate(true, false));
        assert!(or_gate(false, true));
        assert!(!or_gate(false, false));
    }

    #[test]
    fn xor_gate_table() {
        assert!(!xor_gate(true, true));
        assert!(xor_gate(true, false));
        assert!(xor_gate(false, true));
        assert!(!xor_gate(false, false));
    }

    #[test]
    fn xnor_gate_table() {
        assert!(xnor_gate(true, true));
        assert!(!xnor_gate(true, false));
        assert!(!xnor_gate(false, true));
        assert!(xnor_gate(false, false));
    }

    #[test]
    fn implication_gate_table() {
        assert!(implication_gate(true, true));
        assert!(!implication_gate(true, false));
        assert!(implication_gate(false, true));
        assert!(implication_gate(false, false));
    }
}
