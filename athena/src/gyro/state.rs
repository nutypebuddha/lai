//! # Gyro State — orientation and angular velocity
//!
//! The gyroscope maintains a spinning wheel with 12 signs at 30° intervals.
//! Orientation is a continuous angle [0, 360°). Angular velocity in signs/second.
//!
//! ## Calibration Flow
//!
//! The gyro receives torque from two sources:
//! 1. **Token torque** — individual token mass via `apply_torque()`
//! 2. **Matrix torque** — settling matrix aggregate via `apply_matrix()`
//!
//! After each update, `alignment_weights()` determines which primitives fire.

use crate::astrology::Sign;
use crate::descent::SettlingMatrix;

/// Orientation on the zodiac wheel (degrees, 0 = Aries, 30 = Taurus, etc.)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Orientation(pub f64);

impl Orientation {
    /// Create new orientation, normalized to [0, 360)
    pub fn new(degrees: f64) -> Self {
        let normalized = degrees.rem_euclid(360.0);
        Orientation(normalized)
    }

    /// Get the dominant sign at this orientation
    pub fn dominant_sign(&self) -> Sign {
        let index = ((self.0 / 30.0).round() as i32).rem_euclid(12) as usize;
        Sign::from_index(index)
    }

    /// Angular distance to another orientation (shortest path, degrees)
    pub fn distance_to(&self, other: Orientation) -> f64 {
        let diff = (other.0 - self.0).abs();
        diff.min(360.0 - diff)
    }
}

/// 3D axis for gyroscope dynamics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    X, // Precession axis (in-plane)
    Y, // Nutation axis (out-of-plane)
    Z, // Spin axis (wheel normal)
}

/// Complete gyroscope state
#[derive(Debug, Clone)]
pub struct GyroState {
    /// Current orientation on the wheel
    pub orientation: Orientation,
    /// Angular velocity (degrees/second, positive = counterclockwise)
    pub angular_velocity: f64,
    /// Accumulated torque from token mass
    pub torque_accumulator: f64,
    /// Moment of inertia (resistance to angular acceleration)
    pub moment_of_inertia: f64,
    /// Damping coefficient (friction)
    pub damping: f64,
    /// Mass distribution around the wheel (12 signs)
    pub mass_distribution: [f64; 12],
}

impl Default for GyroState {
    fn default() -> Self {
        Self::new()
    }
}

impl GyroState {
    /// Create a new gyroscope at rest, Aries at 0°
    pub fn new() -> Self {
        GyroState {
            orientation: Orientation::new(0.0),
            angular_velocity: 0.0,
            torque_accumulator: 0.0,
            moment_of_inertia: 1.0,
            damping: 0.1,
            mass_distribution: [0.0; 12],
        }
    }

    /// Apply torque at a specific sign (from token mass)
    ///
    /// Torque = mass * lever_arm. Lever arm is sin(angle between sign and orientation).
    pub fn apply_torque(&mut self, sign: Sign, mass: f64) {
        let sign_angle = sign.index() as f64 * 30.0;
        let orientation_angle = self.orientation.0;
        let lever_arm = (sign_angle - orientation_angle).to_radians().sin().abs();
        let torque = mass * lever_arm;
        self.torque_accumulator += torque;
        // Add mass to distribution
        self.mass_distribution[sign.index()] += mass;
    }

    /// Apply torque from a settling matrix — the calibrated aggregate of all tokens.
    ///
    /// Each dominant sign in the matrix applies mass proportional to its
    /// activation. This connects the descent resolution to gyroscopic precession.
    pub fn apply_matrix(&mut self, matrix: &SettlingMatrix) {
        // Apply torque from each token's dominant sign
        for token in &matrix.tokens {
            if let Some(sign) = token.western_classification.dominant_sign() {
                let mass = token.confidence.max(0.1);
                self.apply_torque(sign, mass);
            }
        }

        // Apply additional torque from aggregate classification
        if let Some(sign) = matrix.aggregate_western.dominant_sign() {
            let aggregate_mass = matrix.resolution_score;
            self.apply_torque(sign, aggregate_mass.max(0.2));
        }
    }

    /// Update gyroscope state by time step dt (seconds)
    pub fn update(&mut self, dt: f64) {
        // Angular acceleration = torque / moment_of_inertia
        let angular_accel = self.torque_accumulator / self.moment_of_inertia;

        // Update velocity with acceleration and exponential damping
        self.angular_velocity += angular_accel * dt;
        self.angular_velocity *= (-self.damping * dt).exp();

        // Update orientation
        let delta = self.angular_velocity * dt;
        self.orientation = Orientation::new(self.orientation.0 + delta);

        // Clear torque accumulator (impulse applied)
        self.torque_accumulator = 0.0;
    }

    /// Get the current dominant sign
    pub fn current_sign(&self) -> Sign {
        self.orientation.dominant_sign()
    }

    /// Get the dominant sign and its mass from the mass distribution
    pub fn dominant_sign_info(&self) -> (Sign, f64) {
        let mut max_mass = 0.0f64;
        let mut max_idx = 0usize;
        for (i, &mass) in self.mass_distribution.iter().enumerate() {
            if mass > max_mass {
                max_mass = mass;
                max_idx = i;
            }
        }
        (Sign::from_index(max_idx), max_mass)
    }

    /// Get current precession rate (angular velocity in degrees/second)
    pub fn precession(&self) -> f64 {
        self.angular_velocity
    }

    /// Get alignment strength for each primitive at current orientation
    /// Returns 12 weights (one per sign) indicating how aligned each is
    pub fn alignment_weights(&self) -> [f64; 12] {
        let mut weights = [0.0; 12];
        let current = self.orientation.0;
        for (i, weight) in weights.iter_mut().enumerate() {
            let sign_angle = i as f64 * 30.0;
            let diff = (sign_angle - current)
                .abs()
                .min(360.0 - (sign_angle - current).abs());
            // Cosine falloff: 1.0 at exact alignment, 0.0 at 90°, -1.0 at 180°
            *weight = (diff.to_radians()).cos().max(0.0);
        }
        weights
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descent::SettlingMatrix;

    fn make_token_with_sign(
        text: &str,
        sign: crate::astrology::Sign,
    ) -> crate::descent::SettledToken {
        let mut token = crate::descent::SettledToken::new(text);
        token.western_classification =
            crate::astrology::AtomClassification::new().with_sign(sign, 1.0);
        token.confidence = 0.8;
        token
    }

    #[test]
    fn test_apply_matrix_updates_mass_distribution() {
        let mut gyro = GyroState::new();
        // Create a settling matrix with a single Taurus token
        let tokens = vec![make_token_with_sign(
            "force",
            crate::astrology::Sign::Taurus,
        )];
        let matrix = SettlingMatrix::new(tokens);
        gyro.apply_matrix(&matrix);
        // Mass should have been added at Taurus index (1)
        assert!(gyro.mass_distribution[1] > 0.0);
    }

    #[test]
    fn test_apply_matrix_triggers_precession() {
        let mut gyro = GyroState::new();
        let tokens = vec![make_token_with_sign(
            "force",
            crate::astrology::Sign::Taurus,
        )];
        let matrix = SettlingMatrix::new(tokens);
        gyro.apply_matrix(&matrix);
        let initial_orientation = gyro.orientation.0;
        gyro.update(1.0);
        // Orientation should have changed (precession)
        assert!(
            (gyro.orientation.0 - initial_orientation).abs() > 1e-6 || gyro.angular_velocity > 0.0
        );
    }
}
