#[derive(Debug, Clone)]
pub struct BallEconomy {
    pub tray: u64,
    pub spent: u64,
    pub won: u64,
    pub cost_per_gate: f64,
    pub total_cost: f64,
}

impl BallEconomy {
    pub fn new(initial_tray: u64) -> Self {
        BallEconomy {
            tray: initial_tray,
            spent: 0,
            won: 0,
            cost_per_gate: 0.000001,
            total_cost: 0.0,
        }
    }

    pub fn with_cost(mut self, cost_per_gate: f64) -> Self {
        self.cost_per_gate = cost_per_gate;
        self
    }

    pub fn can_afford(&self, num_gates: usize) -> bool {
        let cost = num_gates as u64;
        self.tray >= cost
    }

    pub fn spend(&mut self, num_gates: usize) -> bool {
        let cost = num_gates as u64;
        if self.tray >= cost {
            self.tray -= cost;
            self.spent += cost;
            self.total_cost += cost as f64 * self.cost_per_gate;
            true
        } else {
            false
        }
    }

    pub fn win(&mut self, amount: u64) {
        self.tray += amount;
        self.won += amount;
    }

    pub fn refund(&mut self, amount: u64) {
        self.tray += amount;
        self.spent = self.spent.saturating_sub(amount);
    }

    pub fn balance(&self) -> u64 {
        self.tray
    }

    pub fn total_spent(&self) -> u64 {
        self.spent
    }

    pub fn total_won(&self) -> u64 {
        self.won
    }

    pub fn total_cost_usd(&self) -> f64 {
        self.total_cost
    }

    pub fn roi(&self) -> f64 {
        if self.spent == 0 {
            return 0.0;
        }
        (self.won as f64 / self.spent as f64) * 100.0
    }

    pub fn is_bust(&self) -> bool {
        self.tray == 0
    }

    pub fn reset(&mut self, initial: u64) {
        self.tray = initial;
        self.spent = 0;
        self.won = 0;
        self.total_cost = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ball_economy_new() {
        let econ = BallEconomy::new(100);
        assert_eq!(econ.balance(), 100);
        assert_eq!(econ.spent, 0);
    }

    #[test]
    fn test_ball_economy_spend() {
        let mut econ = BallEconomy::new(100);
        assert!(econ.spend(30));
        assert_eq!(econ.balance(), 70);
        assert_eq!(econ.spent, 30);
        assert!(!econ.spend(80));
        assert_eq!(econ.balance(), 70);
    }

    #[test]
    fn test_ball_economy_win() {
        let mut econ = BallEconomy::new(100);
        econ.spend(50);
        econ.win(20);
        assert_eq!(econ.balance(), 70);
        assert_eq!(econ.total_won(), 20);
    }

    #[test]
    fn test_ball_economy_refund() {
        let mut econ = BallEconomy::new(100);
        econ.spend(50);
        econ.refund(30);
        assert_eq!(econ.balance(), 80);
        assert_eq!(econ.spent, 20);
    }

    #[test]
    fn test_ball_economy_roi() {
        let mut econ = BallEconomy::new(100);
        econ.spend(40);
        econ.win(80);
        assert!((econ.roi() - 200.0).abs() < 0.01);
    }

    #[test]
    fn test_ball_economy_roi_zero_spend() {
        let econ = BallEconomy::new(100);
        assert_eq!(econ.roi(), 0.0);
    }

    #[test]
    fn test_ball_economy_is_bust() {
        let mut econ = BallEconomy::new(100);
        assert!(!econ.is_bust());
        econ.spend(100);
        assert!(econ.is_bust());
    }

    #[test]
    fn test_ball_economy_reset() {
        let mut econ = BallEconomy::new(100);
        econ.spend(50);
        econ.win(30);
        econ.reset(200);
        assert_eq!(econ.balance(), 200);
        assert_eq!(econ.spent, 0);
    }
}
