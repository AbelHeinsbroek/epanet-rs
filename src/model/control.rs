use serde::{Deserialize, Serialize};
use crate::model::link::LinkStatus;
use crate::constants::{PSIperFT, H_TOL};
use crate::solver::SolverState;
use crate::model::network::Network;

#[derive(Debug, Deserialize, Serialize)]
pub enum ControlCondition {
  HighPressure { node_index: usize, target: f64 },
  LowPressure { node_index: usize, target: f64 },
  HighLevel { tank_index: usize, target: f64 },
  LowLevel { tank_index: usize, target: f64 },
  Time { seconds: usize },
  ClockTime { seconds: usize },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Control {
  pub condition: ControlCondition,
  pub link_id: Box<str>,
  pub setting: Option<f64>,
  pub status: Option<LinkStatus>
}

impl Control {
  pub fn is_active(&self, state: &SolverState, network: &Network, time: usize, clocktime: usize) -> bool {

    match &self.condition {
      ControlCondition::Time { seconds } => *seconds == time,
      ControlCondition::ClockTime { seconds } => *seconds == clocktime,
      ControlCondition::HighPressure { node_index, target } => {
        let node = &network.nodes[*node_index];
        let value = (state.heads[*node_index] + node.elevation) * PSIperFT; // convert head to pressure
        value - *target >= -H_TOL
      }
      ControlCondition::LowPressure { node_index, target } => {
        let node = &network.nodes[*node_index];
        let value = (state.heads[*node_index] + node.elevation) * PSIperFT; // convert head to pressure
        value - *target <= H_TOL
      }
      ControlCondition::HighLevel { tank_index, target } => {
        let value = state.heads[*tank_index];
        value - *target >= -H_TOL
      }
      ControlCondition::LowLevel { tank_index, target } => {
        let value = state.heads[*tank_index];
        value - *target <= H_TOL
      }
    }
  }

  pub fn activate(&self, state: &mut SolverState, network: &Network) -> bool {
    let link_index = network.link_map.get(&self.link_id).unwrap();

    if let Some(status) = self.status {
      let changed = state.statuses[*link_index] != status;
      state.statuses[*link_index] = status;
      return changed
    }
    if let Some(setting) = self.setting {
      let changed = state.settings[*link_index] != setting;
      state.settings[*link_index] = setting;
      return changed
    }
    return false;
  }

}