use crate::model::units::UnitConversion;
use crate::model::units::{FlowUnits, UnitSystem};
use crate::constants::*;
use crate::model::curve::Curve;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tank {
  pub elevation: f64,        // elevation of the tank (ft)
  pub initial_level: f64,    // initial level of the tank (ft)
  pub min_level: f64,        // minimum level of the tank (ft)
  pub max_level: f64,        // maximum level of the tank (ft)
  pub diameter: f64,        // nominal diameter of the tank (ft)
  pub min_volume: f64,      // minimum volume of the tank (ft^3)
  pub volume_curve_id: Option<Box<str>>, // id of the volume curve
  pub overflow: bool,                   // whether the tank has an overflow
  #[serde(skip)]
  pub volume_curve: Option<Arc<Curve>>, // volume curve
  #[serde(skip)]
  pub links_to: Vec<usize>, // indices of the links connected from N -> Tank
  #[serde(skip)]
  pub links_from: Vec<usize>, // indices of the links connected from Tank -> N
}

impl Tank {

  /// Calculate the time to reach a given level given the head and flow
  pub fn time_to_reach_head(&self, current_head: f64, target_head: f64, flow: f64) -> usize {

    // check if the target head is above the max level or below the min level
    if target_head > self.elevation + self.max_level {
      return usize::MAX;
    }
    if target_head < self.elevation + self.min_level {
      return usize::MAX;
    }

    let current_volume = self.volume_at_head(current_head);
    let target_volume = self.volume_at_head(target_head);

    let delta_volume = target_volume - current_volume;

    if delta_volume == 0.0 {
      return 0;
    }
    if delta_volume > 0.0 && flow > 0.0 {
      return (delta_volume / flow) as usize;
    }
    if delta_volume < 0.0 && flow < 0.0 {
      return (delta_volume / flow) as usize;
    }

    return usize::MAX;
  }
  /// calculate the time to fill or drain the tank given the head and flow
  pub fn time_to_fill_or_drain(&self, head: f64, flow: f64) -> usize {

    if flow == 0.0 {
      return usize::MAX;
    }

    let max_head = self.elevation + self.max_level;
    let min_head = self.elevation + self.min_level;

    return self.time_to_reach_head(head, max_head, flow).min(self.time_to_reach_head(head, min_head, flow));

  }

  pub fn volume_at_head(&self, head: f64) -> f64 {
    let level = head - self.elevation;

    // if the level is negative, return 0.0
    if level < 0.0 {
      return 0.0;
    }

    if self.volume_curve.is_some() {
      panic!("Tank volume curves not supported yet!");
    }
    else {
      return level * PI * self.diameter * self.diameter / 4.0;
    }
  }
  pub fn min_volume(&self) -> f64 {
    if self.volume_curve.is_some() {
      panic!("Tank volume curves not supported yet!");
    }
    else {
      return self.min_level * PI * self.diameter * self.diameter / 4.0;
    }
  }
  pub fn max_volume(&self) -> f64 {
    if self.volume_curve.is_some() {
      panic!("Tank volume curves not supported yet!");
    }
    else {
      return self.max_level * PI * self.diameter * self.diameter / 4.0;
    }
  }

  pub fn new_head(&self, delta_volume: f64, current_head: f64) -> f64 {

    let mut level = current_head - self.elevation;

    if self.volume_curve.is_some() {
      panic!("Tank volume curves not supported yet!");
    }
    else {
      // linear volume curve
      let surface_area = PI * self.diameter * self.diameter / 4.0; // in ft^2
      let new_level = level + delta_volume / surface_area; // in ft

      level = new_level.clamp(self.min_level, self.max_level);
    }

    return self.elevation + level;
  }
}

impl UnitConversion for Tank {
  fn convert_units(&mut self, _flow: &FlowUnits, system: &UnitSystem, _reverse: bool) {
    // convert the initial level, min level, max level, diameter, min volume
    if system == &UnitSystem::SI {
      self.initial_level = self.initial_level * MperFT;
      self.min_level = self.min_level * MperFT;
      self.max_level = self.max_level * MperFT;
      self.diameter = self.diameter * MperFT;
      self.min_volume = self.min_volume * M3perFT3;
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  fn test_tank() -> Tank {
    Tank {
      elevation: 10.0,
      initial_level: 10.0,
      min_level: 5.0,
      max_level: 20.0,
      diameter: (10.0 / (PI * 0.25)).sqrt(), // 10 ft^2 surface area
      min_volume: 0.0,
      volume_curve_id: None,
      overflow: false,
      volume_curve: None,
      links_to: vec![],
      links_from: vec![],
    }
  }

  #[test]
  fn test_volume_at_head() {
    let tank = test_tank();
    assert!(tank.volume_at_head(11.0) - 10.0 < 1e-6);
    assert!(tank.volume_at_head(5.0) - 0.0 < 1e-6);

    assert!(tank.max_volume() - 200.0 < 1e-6);
    assert!(tank.min_volume() - 50.0 < 1e-6);
  }

  #[test]
  fn test_time_to_reach_level() {
    let tank = test_tank();
    let flow = 1.0; // in ft^3/s
    // time to reach the target head is 10 seconds
    let time = tank.time_to_reach_head(20.0, 21.0, flow);
    assert_eq!(time, 10);

    // time to reach the target head is 10 seconds with negative flow
    let time = tank.time_to_reach_head(20.0, 19.0, -flow);
    assert_eq!(time, 10);

    // time to reach the target head is infinite because the target head is lower than the current head with positive flow
    let time = tank.time_to_reach_head(20.0, 19.0, flow);
    assert_eq!(time, usize::MAX);

    // time to reach a target head > max level is infinite
    let time = tank.time_to_reach_head(20.0, 31.0, flow);
    assert_eq!(time, usize::MAX);

    // time to reach a target head < min level is infinite
    let time = tank.time_to_reach_head(20.0, 0.0, -flow);
    assert_eq!(time, usize::MAX);

    // time to fill or drain the tank is 100 seconds with positive flow
    let time = tank.time_to_fill_or_drain(20.0, flow);
    assert_eq!(time, 100);

    // time to fill or drain the tank is 50 seconds with negative flow
    let time = tank.time_to_fill_or_drain(20.0, -flow);
    assert_eq!(time, 50);

    // time to fill or drain the tank is infinite with zero flow
    let time = tank.time_to_fill_or_drain(20.0, 0.0);
    assert_eq!(time, usize::MAX);
  }




}

