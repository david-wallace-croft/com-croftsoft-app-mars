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

pub struct StateDiscriminantCooling;
pub struct StateDiscriminantExploding;
pub struct StateDiscriminantNominal;

pub struct StateTransition<D> {
  _state_discriminant: D,
}

impl StateTransition<StateDiscriminantCooling> {
  pub fn to_nominal(&self) -> State {
    State::Nominal(StateTransition {
      _state_discriminant: StateDiscriminantNominal {},
    })
  }
}

impl StateTransition<StateDiscriminantExploding> {
  pub fn to_cooling(&self) -> State {
    State::Cooling(StateTransition {
      _state_discriminant: StateDiscriminantCooling {},
    })
  }
}

impl StateTransition<StateDiscriminantNominal> {
  pub fn to_exploding(&self) -> State {
    State::Exploding(StateTransition {
      _state_discriminant: StateDiscriminantExploding {},
    })
  }
}

pub enum State {
  Cooling(StateTransition<StateDiscriminantCooling>),
  Exploding(StateTransition<StateDiscriminantExploding>),
  Nominal(StateTransition<StateDiscriminantNominal>),
}

impl Default for State {
  fn default() -> Self {
    State::Nominal(StateTransition {
      _state_discriminant: StateDiscriminantNominal {},
    })
  }
}
