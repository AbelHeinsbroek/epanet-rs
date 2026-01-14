use crate::model::units::UnitConversion;
use crate::model::units::{FlowUnits, UnitSystem};

#[derive(Debug)]
pub struct Tank {

}

impl UnitConversion for Tank {
  fn convert_units(&mut self, _flow: &FlowUnits, _system: &UnitSystem, _reverse: bool) {
    panic!("Tank units conversion not yet implemented");
  }
}