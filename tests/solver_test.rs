//! Integration test for the hydraulic solver using pump.inp

use epanet_rs::model::network::Network;
use epanet_rs::solver::HydraulicSolver;

/// Test solving pump.inp and verify exact head and flow values
#[test]
fn test_solve_pump_network() {
    let mut network = Network::default();
    network.read_inp("networks/pump.inp").expect("Failed to load pump.inp");

    let solver = HydraulicSolver::new(&network);
    let result = solver.run(false, false);

    // Expected heads (in feet)
    let expected_heads: Vec<(&str, f64)> = vec![
        ("1", 166.00),
        ("2", 164.35),
        ("3", 164.61),
        ("4", 163.76),
        ("5", 163.67),
        ("6", 163.05),
        ("7", 162.95),
        ("FH", 100.00),
        ("FH2", 85.00),
    ];

    // Expected flows (in CFS)
    let expected_flows: Vec<(&str, f64)> = vec![
        ("B", 4.29),
        ("C", 4.71),
        ("D", 2.71),
        ("E", 3.29),
        ("F", 1.00),
        ("G", 3.00),
        ("H", 1.00),
        ("I", 0.00),
        ("1", 10.00),  // pump
    ];

    // Verify heads
    for (node_id, expected_head) in &expected_heads {
        let idx = *network.node_map.get(*node_id).expect(&format!("Node {} not found", node_id));
        let actual_head = result.heads[0][idx];
        assert!(
            (actual_head - expected_head).abs() < 0.01,
            "Head mismatch for node {}: expected {:.2}, got {:.2}",
            node_id, expected_head, actual_head
        );
    }

    // Verify flows
    for (link_id, expected_flow) in &expected_flows {
        let idx = *network.link_map.get(*link_id).expect(&format!("Link {} not found", link_id));
        let actual_flow = result.flows[0][idx];
        assert!(
            (actual_flow - expected_flow).abs() < 0.01,
            "Flow mismatch for link {}: expected {:.2}, got {:.2}",
            link_id, expected_flow, actual_flow
        );
    }
}
