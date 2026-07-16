//! # Traversal — following concepts through the wheel
//!
//! A traversal starts at a domain and follows edges through the wheel,
//! collecting formulas at each node. The result is a path of domains
//! with the formulas that connect them.

use std::collections::HashSet;

use serde::Serialize;

use crate::formula::FormulaRegistry;
use crate::wheel::{Aspect, Domain, WheelGraph};

/// A traversal through the zodiac wheel.
///
/// Records the path taken and the formulas discovered at each step.
/// Formula IDs are derived from `path[].formulas_at_node` — not stored separately.
#[derive(Debug, Clone, Serialize)]
pub struct Traversal {
    pub start: Domain,
    pub max_depth: usize,
    pub path: Vec<TraversalStep>,
    pub explored: Vec<Domain>,
}

/// A single step in a traversal.
#[derive(Debug, Clone, Serialize)]
pub struct TraversalStep {
    pub domain: Domain,
    pub aspect_entered: Option<Aspect>,
    pub formulas_at_node: Vec<String>,
}

impl Traversal {
    /// Create a new traversal from a starting domain.
    pub fn new(
        wheel: &WheelGraph,
        formulas: &FormulaRegistry,
        start: Domain,
        max_depth: usize,
    ) -> Self {
        let mut traversal = Traversal {
            start,
            max_depth,
            path: Vec::new(),
            explored: Vec::new(),
        };
        let mut explored_set: HashSet<Domain> = HashSet::new();

        // Collect formulas at the starting node
        let start_formulas = formulas.by_domain(start);
        let start_ids: Vec<String> = start_formulas.iter().map(|f| f.id.clone()).collect();

        traversal.path.push(TraversalStep {
            domain: start,
            aspect_entered: None,
            formulas_at_node: start_ids.clone(),
        });
        traversal.explored.push(start);
        explored_set.insert(start);

        // BFS to explore neighbors
        let mut queue: std::collections::VecDeque<(Domain, usize)> = wheel
            .neighbors(start, None)
            .into_iter()
            .map(|n| (n, 1))
            .collect();

        while let Some((current, depth)) = queue.pop_front() {
            if depth >= max_depth || explored_set.contains(&current) {
                continue;
            }

            let aspect =
                Aspect::between(traversal.explored.last().copied().unwrap_or(start), current);
            let node_formulas = formulas.by_domain(current);
            let node_ids: Vec<String> = node_formulas.iter().map(|f| f.id.clone()).collect();

            traversal.path.push(TraversalStep {
                domain: current,
                aspect_entered: Some(aspect),
                formulas_at_node: node_ids.clone(),
            });
            traversal.explored.push(current);
            explored_set.insert(current);

            // Add neighbors
            if depth + 1 < max_depth {
                for neighbor in wheel.neighbors(current, None) {
                    if !explored_set.contains(&neighbor) {
                        queue.push_back((neighbor, depth + 1));
                    }
                }
            }
        }

        traversal
    }

    /// Get the domains visited in this traversal.
    pub fn domains_visited(&self) -> Vec<Domain> {
        self.path.iter().map(|s| s.domain).collect()
    }

    /// Get the aspects traversed between consecutive domains.
    pub fn aspects_traversed(&self) -> Vec<Option<Aspect>> {
        self.path.iter().map(|s| s.aspect_entered).collect()
    }

    /// Total number of formulas discovered (summed across all steps).
    pub fn formula_count(&self) -> usize {
        self.path.iter().map(|s| s.formulas_at_node.len()).sum()
    }

    /// Whether a specific domain was visited.
    pub fn visited(&self, domain: Domain) -> bool {
        self.explored.contains(&domain)
    }

    /// Number of unique domains visited.
    pub fn domains_visited_count(&self) -> usize {
        self.explored.len()
    }

    /// Whether the traversal is complete (exhausted all reachable nodes).
    pub fn is_complete(&self) -> bool {
        self.explored.len() >= crate::wheel::ALL_DOMAINS.len() || self.path.len() >= self.max_depth
    }

    /// Format the traversal as a readable path.
    pub fn format_path(&self) -> String {
        let mut s = String::new();
        for (i, step) in self.path.iter().enumerate() {
            if i > 0 {
                if let Some(aspect) = step.aspect_entered {
                    s.push_str(&format!(" —{:?}→ ", aspect));
                } else {
                    s.push_str(" → ");
                }
            }
            s.push_str(&format!(
                "{}{}",
                step.domain.symbol(),
                step.domain.full_name()
            ));
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formula::{Formula, FormulaRegistry};
    use crate::wheel::Domain;

    fn setup() -> (WheelGraph, FormulaRegistry) {
        let wheel = WheelGraph::new();
        let mut registry = FormulaRegistry::new();
        registry
            .register_all(vec![
                Formula::atomic("f1", Domain::Mangala, vec![], "x", "1.0", "test"),
                Formula::atomic("f2", Domain::Shukra, vec![], "y", "2.0", "test"),
            ])
            .unwrap();
        (wheel, registry)
    }

    #[test]
    fn test_traversal_starts_at_domain() {
        let (wheel, registry) = setup();
        let t = Traversal::new(&wheel, &registry, Domain::Mangala, 3);
        assert_eq!(t.start, Domain::Mangala);
        assert!(!t.path.is_empty());
        assert_eq!(t.path[0].domain, Domain::Mangala);
    }

    #[test]
    fn test_traversal_discovers_formulas() {
        let (wheel, registry) = setup();
        let t = Traversal::new(&wheel, &registry, Domain::Mangala, 5);
        assert!(t.formula_count() >= 1);
    }

    #[test]
    fn test_traversal_expands_neighbors() {
        let (wheel, registry) = setup();
        let t = Traversal::new(&wheel, &registry, Domain::Mangala, 2);
        assert!(!t.explored.is_empty());
    }

    #[test]
    fn test_format_path() {
        let (wheel, registry) = setup();
        let t = Traversal::new(&wheel, &registry, Domain::Mangala, 1);
        let formatted = t.format_path();
        assert!(formatted.contains("Mangala"));
    }
}
