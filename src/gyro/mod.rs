/// Pure function: Compute rotation angle for a graha at given position.
pub fn compute_rotation_angle(position: u8, total_positions: u8) -> f64 {
    (position as f64 / total_positions as f64) * 360.0
}

/// Pure function: Check if two angles are within tolerance.
pub fn angles_within_tolerance(left_angle: f64, right_angle: f64, tolerance: f64) -> bool {
    let difference = (left_angle - right_angle).abs();
    difference <= tolerance || (360.0 - difference) <= tolerance
}

/// Pure function: Map a graha to its wheel position (0-8 for 9-graha).
pub fn map_graha_to_position(graha_index: u8) -> u8 {
    graha_index % 9
}

/// Pure function: Compute next position in gyroscopic rotation.
pub fn compute_next_position(current_position: u8, step: u8, total_positions: u8) -> u8 {
    (current_position + step) % total_positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_rotation_angle_basic() {
        assert_eq!(compute_rotation_angle(0, 9), 0.0);
        assert!((compute_rotation_angle(1, 9) - 40.0).abs() < f64::EPSILON);
        assert!((compute_rotation_angle(9, 9) - 360.0).abs() < f64::EPSILON);
    }

    #[test]
    fn angles_within_tolerance_basic() {
        assert!(angles_within_tolerance(0.0, 0.0, 1.0));
        assert!(angles_within_tolerance(0.0, 0.5, 1.0));
        assert!(!angles_within_tolerance(0.0, 2.0, 1.0));
        assert!(angles_within_tolerance(359.0, 1.0, 2.0));
    }

    #[test]
    fn map_graha_to_position_basic() {
        assert_eq!(map_graha_to_position(0), 0);
        assert_eq!(map_graha_to_position(8), 8);
        assert_eq!(map_graha_to_position(9), 0);
    }

    #[test]
    fn compute_next_position_basic() {
        assert_eq!(compute_next_position(0, 1, 9), 1);
        assert_eq!(compute_next_position(8, 1, 9), 0);
        assert_eq!(compute_next_position(0, 3, 9), 3);
    }
}
