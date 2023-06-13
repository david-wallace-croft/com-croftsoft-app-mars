// =============================================================================
//! - Default Ammo Dump State for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-05
//! - Updated: 2023-06-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constant::AMMO_DUMP_COOLING_TIME_SECONDS;

pub struct FromCooling {
  cooling_time_elapsed_seconds: f64,
}

pub struct FromExploding;

pub struct FromNominal;

pub struct Transition<F> {
  pub from: F,
}

impl Transition<FromCooling> {
  pub fn done_cooling(
    &mut self,
    time_delta: f64,
  ) -> bool {
    self.from.cooling_time_elapsed_seconds += time_delta;
    self.from.cooling_time_elapsed_seconds >= AMMO_DUMP_COOLING_TIME_SECONDS
  }

  pub fn to_nominal(&self) -> State {
    State::Nominal(Transition {
      from: FromNominal {},
    })
  }
}

impl Transition<FromExploding> {
  pub fn to_cooling(&self) -> State {
    State::Cooling(Transition {
      from: FromCooling {
        cooling_time_elapsed_seconds: 0.,
      },
    })
  }
}

impl Transition<FromNominal> {
  pub fn to_exploding(&self) -> State {
    State::Exploding(Transition {
      from: FromExploding {},
    })
  }
}

pub enum State {
  Cooling(Transition<FromCooling>),
  Exploding(Transition<FromExploding>),
  Nominal(Transition<FromNominal>),
}

impl Default for State {
  fn default() -> Self {
    State::Nominal(Transition {
      from: FromNominal {},
    })
  }
}
