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

#[derive(Clone, Copy)]
pub struct StateDiscriminantCooling;

#[derive(Clone, Copy)]
pub struct StateDiscriminantExploding;

#[derive(Clone, Copy)]
pub struct StateDiscriminantNominal;

#[derive(Clone, Copy)]
pub struct StateOperator<S> {
  _state_discriminant: S,
}

impl StateOperator<StateDiscriminantCooling> {
  pub fn to_nominal(self) -> State {
    State::Nominal(StateOperator {
      _state_discriminant: StateDiscriminantNominal {},
    })
  }
}

impl StateOperator<StateDiscriminantExploding> {
  pub fn to_cooling(self) -> State {
    State::Cooling(StateOperator {
      _state_discriminant: StateDiscriminantCooling {},
    })
  }
}

impl StateOperator<StateDiscriminantNominal> {
  pub fn to_exploding(self) -> State {
    State::Exploding(StateOperator {
      _state_discriminant: StateDiscriminantExploding {},
    })
  }
}

pub enum State {
  Cooling(StateOperator<StateDiscriminantCooling>),
  Exploding(StateOperator<StateDiscriminantExploding>),
  Nominal(StateOperator<StateDiscriminantNominal>),
}

impl Default for State {
  fn default() -> Self {
    State::Nominal(StateOperator {
      _state_discriminant: StateDiscriminantNominal {},
    })
  }
}
