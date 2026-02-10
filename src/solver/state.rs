use crate::model::link::{LinkTrait, LinkStatus};
use crate::model::network::Network;

/// The solver state is the initial/final state of the solver for a single step
#[derive(Debug, Clone)]
pub struct SolverState {
  pub flows: Vec<f64>,
  pub heads: Vec<f64>,
  pub demands: Vec<f64>,
  pub statuses: Vec<LinkStatus>,
  pub settings: Vec<f64>,
  pub resistances: Vec<f64>,
}

impl SolverState {
  /// Create a new solver state with the initial values for the flows, heads, demands and statuses and calculate resistances
  pub fn new_with_initial_values(network: &Network) -> Self {
    Self { flows: network.links.iter().map(|l| l.initial_flow()).collect::<Vec<f64>>(), 
           heads: network.nodes.iter().map(|n| n.initial_head()).collect::<Vec<f64>>(), 
           demands: vec![0.0; network.nodes.len()], 
           settings: network.links.iter().map(|l| l.initial_setting()).collect::<Vec<f64>>(),
           statuses: network.links.iter().map(|l| l.initial_status).collect::<Vec<LinkStatus>>(),
           resistances: network.links.iter().map(|l| l.resistance()).collect::<Vec<f64>>(),
         }
  }
}