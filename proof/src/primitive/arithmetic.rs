use super::nand::{and_gate, or_gate, xor_gate};

/// Pure function: Half adder. Returns (sum, carry) from two boolean inputs.
pub fn half_adder(left_operand: bool, right_operand: bool) -> (bool, bool) {
    let sum = xor_gate(left_operand, right_operand);
    let carry = and_gate(left_operand, right_operand);
    (sum, carry)
}

/// Pure function: Full adder. Returns (sum, carry) including carry input.
pub fn full_adder(left_operand: bool, right_operand: bool, carry_input: bool) -> (bool, bool) {
    let (partial_sum, partial_carry) = half_adder(left_operand, right_operand);
    let (sum, final_carry) = half_adder(partial_sum, carry_input);
    let carry_output = or_gate(partial_carry, final_carry);
    (sum, carry_output)
}

/// Pure function: Add two u8 values using only NAND-derived logic.
pub fn add_unsigned_8(left_operand: u8, right_operand: u8) -> u8 {
    let mut result = 0u8;
    let mut carry = false;

    for bit_position in 0..8 {
        let left_bit = (left_operand >> bit_position) & 1 == 1;
        let right_bit = (right_operand >> bit_position) & 1 == 1;
        let (sum, next_carry) = full_adder(left_bit, right_bit, carry);
        if sum {
            result |= 1 << bit_position;
        }
        carry = next_carry;
    }

    result
}

/// Pure function: Negate a u8 (two's complement).
pub fn negate_unsigned_8(value: u8) -> u8 {
    add_unsigned_8(!value, 1)
}

/// Pure function: Subtract two u8 values using NAND-derived arithmetic.
pub fn subtract_unsigned_8(left_operand: u8, right_operand: u8) -> u8 {
    add_unsigned_8(left_operand, negate_unsigned_8(right_operand))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_adder_no_carry() {
        let (sum, carry) = half_adder(false, false);
        assert!(!sum);
        assert!(!carry);
    }

    #[test]
    fn half_adder_with_carry() {
        let (sum, carry) = half_adder(true, true);
        assert!(!sum);
        assert!(carry);
    }

    #[test]
    fn full_adder_all_combinations() {
        let cases = [
            ((false, false, false), (false, false)),
            ((true, false, false), (true, false)),
            ((true, true, false), (false, true)),
            ((true, true, true), (true, true)),
        ];
        for ((a, b, c_in), (expected_sum, expected_carry)) in cases {
            let (sum, carry) = full_adder(a, b, c_in);
            assert_eq!(sum, expected_sum, "sum failed for {a}, {b}, {c_in}");
            assert_eq!(carry, expected_carry, "carry failed for {a}, {b}, {c_in}");
        }
    }

    #[test]
    fn add_unsigned_8_basic() {
        assert_eq!(add_unsigned_8(0, 0), 0);
        assert_eq!(add_unsigned_8(1, 0), 1);
        assert_eq!(add_unsigned_8(1, 1), 2);
        assert_eq!(add_unsigned_8(255, 1), 0);
        assert_eq!(add_unsigned_8(42, 58), 100);
        assert_eq!(add_unsigned_8(100, 55), 155);
    }

    #[test]
    fn negate_unsigned_8_basic() {
        assert_eq!(negate_unsigned_8(0), 0);
        assert_eq!(negate_unsigned_8(1), 255);
        assert_eq!(negate_unsigned_8(42), 214);
    }

    #[test]
    fn subtract_unsigned_8_basic() {
        assert_eq!(subtract_unsigned_8(10, 5), 5);
        assert_eq!(subtract_unsigned_8(5, 10), 251);
        assert_eq!(subtract_unsigned_8(100, 100), 0);
    }
}
