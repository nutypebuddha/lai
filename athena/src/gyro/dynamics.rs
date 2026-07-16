//! # Gyro Dynamics — precession, nutation, and torque physics
//!
//! Models the gyroscope as a physical spinning wheel. Token mass applies
//! torque, causing precession (in-plane rotation) and nutation (wobble).

use std::collections::VecDeque;

use crate::gyro::state::{GyroState, Orientation};

/// Parameters controlling precession behavior
#[derive(Debug, Clone, Copy)]
pub struct PrecessionParams {
    /// Base precession rate (degrees/second per unit torque)
    pub precession_rate: f64,
    /// Nutation damping (how fast wobble settles)
    pub nutation_damping: f64,
    /// General damping coefficient
    pub damping: f64,
    /// Coupling between signs (how mass at one sign affects neighbors)
    pub coupling_strength: f64,
    /// Maximum angular velocity (signs/second)
    pub max_velocity: f64,
}

impl Default for PrecessionParams {
    fn default() -> Self {
        PrecessionParams {
            precession_rate: 30.0, // 1 sign per second per unit torque
            nutation_damping: 0.5,
            damping: 0.1,
            coupling_strength: 0.1,
            max_velocity: 360.0, // 12 signs/second max
        }
    }
}

/// Gyroscope dynamics engine
#[derive(Debug, Clone)]
pub struct GyroDynamics {
    params: PrecessionParams,
    /// History of orientations for smoothing (VecDeque for O(1) pop_front)
    orientation_history: VecDeque<Orientation>,
    history_len: usize,
}

impl Default for GyroDynamics {
    fn default() -> Self {
        Self::new()
    }
}

impl GyroDynamics {
    pub fn new() -> Self {
        GyroDynamics {
            params: PrecessionParams::default(),
            orientation_history: VecDeque::new(),
            history_len: 10,
        }
    }

    pub fn with_params(params: PrecessionParams) -> Self {
        GyroDynamics {
            params,
            orientation_history: VecDeque::new(),
            history_len: 10,
        }
    }

    /// Step the gyroscope forward by dt seconds
    pub fn step(&mut self, state: &mut GyroState, dt: f64) {
        // Apply precession from accumulated torque
        let precession = state.torque_accumulator * self.params.precession_rate * dt;
        state.angular_velocity += precession;

        // Clamp velocity
        state.angular_velocity = state
            .angular_velocity
            .clamp(-self.params.max_velocity, self.params.max_velocity);

        // Apply damping
        state.angular_velocity *= 1.0 - self.params.damping * dt;

        // Update orientation
        let delta = state.angular_velocity * dt;
        state.orientation = Orientation::new(state.orientation.0 + delta);

        // Record history (VecDeque, O(1) pop_front)
        self.orientation_history.push_back(state.orientation);
        if self.orientation_history.len() > self.history_len {
            self.orientation_history.pop_front();
        }

        // Clear torque
        state.torque_accumulator = 0.0;
    }

    /// Apply impulse from token at sign with given mass
    pub fn apply_impulse(
        &mut self,
        state: &mut GyroState,
        sign: crate::astrology::Sign,
        mass: f64,
    ) {
        state.apply_torque(sign, mass);
    }

    /// Get smoothed orientation (exponential moving average)
    pub fn smoothed_orientation(&self, state: &GyroState) -> Orientation {
        if self.orientation_history.is_empty() {
            return state.orientation;
        }

        let alpha = 0.3; // smoothing factor
        let mut smoothed = state.orientation.0;
        for h in &self.orientation_history {
            smoothed = alpha * h.0 + (1.0 - alpha) * smoothed;
        }
        Orientation::new(smoothed)
    }

    /// Predict orientation after dt seconds
    pub fn predict(&self, state: &GyroState, dt: f64) -> Orientation {
        let predicted_angle = state.orientation.0 + state.angular_velocity * dt;
        Orientation::new(predicted_angle)
    }

    /// Compute alignment strength for all 12 signs
    pub fn alignment(&self, state: &GyroState) -> [f64; 12] {
        let current = state.orientation.0;
        let mut weights = [0.0; 12];
        for (i, weight) in weights.iter_mut().enumerate() {
            let sign_angle = i as f64 * 30.0;
            let diff = (sign_angle - current)
                .abs()
                .min(360.0 - (sign_angle - current).abs());
            // Cosine bell curve centered on current orientation
            *weight = (diff.to_radians()).cos().max(0.0);
        }
        weights
    }

    /// Get dominant sign with confidence
    pub fn dominant(&self, state: &GyroState) -> (crate::astrology::Sign, f64) {
        let weights = self.alignment(state);
        let mut best_idx = 0;
        let mut best_weight = 0.0;
        for (i, &w) in weights.iter().enumerate() {
            if w > best_weight {
                best_weight = w;
                best_idx = i;
            }
        }
        (crate::astrology::Sign::from_index(best_idx), best_weight)
    }
}
