use faer::{Mat, Side};
use faer::prelude::*; // <--- This imports the 'Solve' trait needed for .solve()
use std::f64;

#[derive(Clone, Debug)]
struct Node {
    id: String,
    x: f64,
    y: f64,
    elevation: f64,
    demand: f64,
    head: f64,
    is_fixed: bool,
}

#[derive(Clone, Debug)]
struct Link {
    id: String,
    start: usize,
    end: usize,
    diameter: f64,
    roughness: f64,
    length: f64,
    flow: f64,
}

#[derive(Clone, Debug)]
struct Network {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

impl Network {
  fn solve(&mut self, max_iter: usize, tolerance: f64) {


    for i in 0..max_iter {
        println!("Iteration: {}", i);
    }
  }
}

fn main() {
    let mut network = generate_grid(4);
    network.solve(20, 0.001);

    println!("\n--- Results ---");
    println!("{:<10} | {:<10} | {:<10}", "Node", "Head", "Demand");
    for node in &network.nodes {
        println!("{:<10} | {:<10.3} | {:<10.3}", node.id, node.head, node.demand);
    }
    println!("\n--- Links ---");
    println!("{:<10} | {:<10} | {:<10} | {:<10} | {:<10} | {:<10} | {:<10}", "Link", "Start", "End", "Length", "Diameter", "Roughness", "Flow");
    for link in &network.links {
        println!("{:<10} | {:<10} | {:<10} | {:<10.3} | {:<10.3} | {:<10.3} | {:<10.3}", link.id, link.start, link.end, link.length, link.diameter, link.roughness, link.flow);
    }
}

// --- Helper Functions ---

fn generate_grid(dim: u32) -> Network {
    let mut nodes: Vec<Node> = Vec::new();
    let mut links: Vec<Link> = Vec::new();

    for x in 0..dim {
        for y in 0..dim {
            let is_fixed = x == 0 && y == 0;
            // High initial head helps convergence
            let head = if is_fixed { 100.0 } else { 0.0 };

            nodes.push(Node {
                id: format!("N{}-{}", x, y),
                x: x as f64,
                y: y as f64,
                elevation: 0.0,
                demand: if is_fixed { 0.0 } else { 50.0 },
                head: head,
                is_fixed: is_fixed,
            })
        }
    }
    let mut link_id = 0;

    for x in 0..dim {
        for y in 0..dim {
            let idx = x * dim + y;
            if x < dim - 1 {
                links.push(generate_link(link_id, idx as usize, (idx + dim) as usize));
                link_id += 1;
            }
            if y < dim - 1 {
                links.push(generate_link(link_id, idx as usize, (idx + 1) as usize));
                link_id += 1;
            }
        }
    }
    return Network { nodes, links };
}

fn generate_link(link_id: i32, from: usize, to: usize) -> Link {
    Link {
        id: format!("L{}", link_id),
        start: from,
        end: to,
        length: 100.0,
        diameter: 200.0,
        roughness: 130.0,
        flow: 0.0,
    }
}