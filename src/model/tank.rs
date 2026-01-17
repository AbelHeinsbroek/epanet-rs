use crate::model::units::UnitConversion;
use crate::model::units::{FlowUnits, UnitSystem};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tank {

}

impl UnitConversion for Tank {
  fn convert_units(&mut self, _flow: &FlowUnits, _system: &UnitSystem, _reverse: bool) {
    panic!("Tank units conversion not yet implemented");
  }
}