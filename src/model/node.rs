use crate::model::reservoir::Reservoir;
use crate::model::tank::Tank;
use crate::model::junction::Junction;
use crate::model::units::{FlowUnits, UnitSystem, UnitConversion};

use crate::constants::*;
use serde::{Deserialize, Serialize};

/// Node struct
#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    pub id: Box<str>,
    pub node_type: NodeType,
    pub elevation: f64,
}

/// Node types
#[derive(Debug, Deserialize, Serialize)]
pub enum NodeType {
    Reservoir(Reservoir),
    Tank(Tank),
    Junction(Junction),
}

// helper methods for nodes to check if they are fixed head
// and to get the head pattern 
impl Node {
  pub fn is_fixed(&self) -> bool {
    matches!(self.node_type, NodeType::Reservoir(_) | NodeType::Tank(_))
  }
  pub fn head_pattern(&self) -> Option<&str> {
    if let NodeType::Reservoir(reservoir) = &self.node_type {
      if reservoir.head_pattern.is_some() {
        return Some(reservoir.head_pattern.as_ref().unwrap());
      }
    }
    None
  }
  pub fn initial_head(&self) -> f64 {
    if self.is_fixed() {
      if let NodeType::Tank(tank) = &self.node_type {
        return self.elevation + tank.initial_level;
      }
      return self.elevation;
    }
    return 0.0;
  }
}

impl UnitConversion for Node {
  fn convert_units(&mut self, flow: &FlowUnits, system: &UnitSystem, reverse: bool) {

    // only convert units to/from SI units
    if system == &UnitSystem::SI {
      let scale = if reverse { MperFT } else { 1.0 / MperFT };
      self.elevation = self.elevation * scale;
    }

    match &mut self.node_type {
      NodeType::Reservoir(_reservoir) => (),
      NodeType::Tank(tank) => tank.convert_units(flow, system, reverse),
      NodeType::Junction(junction) => junction.convert_units(flow, system, reverse),
    }
  }
}