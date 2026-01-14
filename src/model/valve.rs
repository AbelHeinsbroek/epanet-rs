use crate::model::link::LinkTrait;
use crate::model::units::{FlowUnits, UnitSystem, UnitConversion};
use crate::constants::*;

#[derive(PartialEq, Eq)]
pub enum ValveType {
  PRV, // Pressure Reducing Valve
  PSV, // Pressure Sensing Valve
  PBV, // Pressure Breaking Valve
  FCV, // Flow Control Valve
  TCV, // Throttle Control Valve
  PCV, // Positional Control Valve
  GPV, // General Purpose Valve
}

#[derive(PartialEq, Eq)]
pub enum ValveStatus {
  Open,
  Closed,
  Active,
}
impl ValveStatus {
  pub fn from_str(status: &str) -> ValveStatus {
    match status.to_uppercase().as_str() {
      "OPEN" => ValveStatus::Open,
      "CLOSED" => ValveStatus::Closed,
      "ACTIVE" => ValveStatus::Active,
      _ => panic!("Invalid valve state")
    }
  }
}

pub struct Valve {
  pub diameter: f64,
  pub setting: f64,
  pub curve: Option<Box<str>>,
  pub valve_type: ValveType,
  pub status: ValveStatus,
}


impl LinkTrait for Valve {
  fn coefficients(&self, q: f64, _resistance: f64) -> (f64, f64) {
    if self.status == ValveStatus::Closed {
      return (1.0/BIG_VALUE, q);
    }
    if self.valve_type == ValveType::PRV || self.valve_type == ValveType::FCV {
      // Get Agadir to run for now
      return (1.0/BIG_VALUE, q);
    }
    (1.0/SMALL_VALUE, q)
  }
  fn resistance(&self) -> f64 {
    0.0
  }
}

impl UnitConversion for Valve {
  fn convert_units(&mut self, _flow: &FlowUnits, system: &UnitSystem, reverse: bool) {
    if system == &UnitSystem::SI {
      if reverse {
        self.diameter = self.diameter * MperFT * 1e3; // convert in to mm
      }
      else {
        self.diameter = self.diameter / 1e3 / MperFT; // convert mm to in
      }
    } else {
      self.diameter = self.diameter / 12.0; // convert in to ft
    }
  }
}