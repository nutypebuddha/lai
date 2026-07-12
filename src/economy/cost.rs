#[derive(Debug, Clone)]
pub struct CostTracker {
    pub gate_costs: Vec<(String, f64)>,
    pub total_cost: f64,
}

impl Default for CostTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl CostTracker {
    pub fn new() -> Self {
        CostTracker {
            gate_costs: Vec::new(),
            total_cost: 0.0,
        }
    }

    pub fn record(&mut self, gate_name: &str, cost: f64) {
        self.gate_costs.push((gate_name.to_string(), cost));
        self.total_cost += cost;
    }

    pub fn average_cost(&self) -> f64 {
        if self.gate_costs.is_empty() {
            return 0.0;
        }
        self.total_cost / self.gate_costs.len() as f64
    }

    pub fn min_cost(&self) -> f64 {
        self.gate_costs
            .iter()
            .map(|(_, c)| *c)
            .fold(f64::INFINITY, f64::min)
    }

    pub fn max_cost(&self) -> f64 {
        self.gate_costs
            .iter()
            .map(|(_, c)| *c)
            .fold(f64::NEG_INFINITY, f64::max)
    }

    pub fn reset(&mut self) {
        self.gate_costs.clear();
        self.total_cost = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_tracker_new() {
        let tracker = CostTracker::new();
        assert_eq!(tracker.gate_costs.len(), 0);
        assert_eq!(tracker.total_cost, 0.0);
    }

    #[test]
    fn test_cost_tracker_record() {
        let mut tracker = CostTracker::new();
        tracker.record("math", 0.001);
        tracker.record("logic", 0.002);
        assert_eq!(tracker.gate_costs.len(), 2);
        assert!((tracker.total_cost - 0.003).abs() < 1e-10);
    }

    #[test]
    fn test_cost_tracker_average() {
        let mut tracker = CostTracker::new();
        tracker.record("math", 0.001);
        tracker.record("logic", 0.003);
        assert!((tracker.average_cost() - 0.002).abs() < 1e-10);
    }

    #[test]
    fn test_cost_tracker_min_max() {
        let mut tracker = CostTracker::new();
        tracker.record("math", 0.001);
        tracker.record("logic", 0.003);
        tracker.record("formal", 0.002);
        assert!((tracker.min_cost() - 0.001).abs() < 1e-10);
        assert!((tracker.max_cost() - 0.003).abs() < 1e-10);
    }

    #[test]
    fn test_cost_tracker_reset() {
        let mut tracker = CostTracker::new();
        tracker.record("math", 0.001);
        tracker.reset();
        assert_eq!(tracker.gate_costs.len(), 0);
        assert_eq!(tracker.total_cost, 0.0);
    }

    #[test]
    fn test_cost_tracker_empty_average() {
        let tracker = CostTracker::new();
        assert_eq!(tracker.average_cost(), 0.0);
    }
}
