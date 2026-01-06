mod input;
mod network;

use std::time::Instant;


fn main() {
  let start_time = Instant::now();
  let network = network::Network::from_inp("grid100.inp");
  let end_time = Instant::now();
  println!("Time taken: {:?}", end_time.duration_since(start_time));
  println!("Loaded network with {} nodes and {} links", network.nodes.len(), network.links.len());
}