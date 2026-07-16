//! Integration tests for the Wheel Graph (Vedic 9-graha wheel)

use athena::wheel::{Aspect, Domain, WheelGraph};

#[test]
fn test_wheel_9_domains() {
    let wheel = WheelGraph::new();
    assert_eq!(wheel.all_nodes().len(), 9);
}

#[test]
fn test_wheel_shortest_path_adjacent() {
    let wheel = WheelGraph::new();
    // Surya (0) and Chandra (1) are adjacent
    let path = wheel.shortest_path(Domain::Surya, Domain::Chandra).unwrap();
    assert_eq!(path.len(), 2);
    assert_eq!(path[0], Domain::Surya);
    assert_eq!(path[1], Domain::Chandra);
}

#[test]
fn test_wheel_aspect_opposite() {
    let wheel = WheelGraph::new();
    // On 9-node wheel: Surya (0) → Brihaspati (4) = 4 steps = opposition
    assert_eq!(
        wheel.aspect_between(Domain::Surya, Domain::Brihaspati),
        Aspect::Opposition
    );
    assert_eq!(
        wheel.aspect_between(Domain::Brihaspati, Domain::Surya),
        Aspect::Opposition
    );
    // Chandra (1) → Shukra (5) = 4 steps = opposition
    assert_eq!(
        wheel.aspect_between(Domain::Chandra, Domain::Shukra),
        Aspect::Opposition
    );
}

#[test]
fn test_wheel_all_opposites() {
    let wheel = WheelGraph::new();
    for node in wheel.all_nodes() {
        let opp = wheel.node(node.domain.opposite());
        assert_eq!(node.opposite, opp.domain);
    }
}
