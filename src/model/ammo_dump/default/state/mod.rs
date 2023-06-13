// =============================================================================
//! - Default Ammo Dump State for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-05
//! - Updated: 2023-06-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constant::AMMO_DUMP_COOLING_TIME_SECONDS;

pub struct CoolingData {
  cooling_time_elapsed_seconds: f64,
}

pub struct ExplodingData;

pub struct NominalData;

pub struct StateOperator<D> {
  data: D,
}

impl StateOperator<CoolingData> {
  pub fn done_cooling(
    &mut self,
    time_delta: f64,
  ) -> bool {
    self.data.cooling_time_elapsed_seconds += time_delta;
    self.data.cooling_time_elapsed_seconds >= AMMO_DUMP_COOLING_TIME_SECONDS
  }

  pub fn to_nominal(&self) -> State {
    State::Nominal(StateOperator {
      data: NominalData {},
    })
  }
}

impl StateOperator<ExplodingData> {
  pub fn to_cooling(&self) -> State {
    State::Cooling(StateOperator {
      data: CoolingData {
        cooling_time_elapsed_seconds: 0.,
      },
    })
  }
}

impl StateOperator<NominalData> {
  pub fn to_exploding(&self) -> State {
    State::Exploding(StateOperator {
      data: ExplodingData {},
    })
  }
}

pub enum State {
  Cooling(StateOperator<CoolingData>),
  Exploding(StateOperator<ExplodingData>),
  Nominal(StateOperator<NominalData>),
}

impl Default for State {
  fn default() -> Self {
    State::Nominal(StateOperator {
      data: NominalData {},
    })
  }
}
