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

pub struct StateSpecificOperator<D> {
  _state_discriminant: D,
}

impl StateSpecificOperator<StateDiscriminantCooling> {
  pub fn to_nominal(&self) -> State {
    State::Nominal(StateSpecificOperator {
      _state_discriminant: StateDiscriminantNominal {},
    })
  }
}

impl StateSpecificOperator<StateDiscriminantExploding> {
  pub fn to_cooling(&self) -> State {
    State::Cooling(StateSpecificOperator {
      _state_discriminant: StateDiscriminantCooling {},
    })
  }
}

impl StateSpecificOperator<StateDiscriminantNominal> {
  pub fn to_exploding(&self) -> State {
    State::Exploding(StateSpecificOperator {
      _state_discriminant: StateDiscriminantExploding {},
    })
  }
}

pub enum State {
  Cooling(StateSpecificOperator<StateDiscriminantCooling>),
  Exploding(StateSpecificOperator<StateDiscriminantExploding>),
  Nominal(StateSpecificOperator<StateDiscriminantNominal>),
}

impl Default for State {
  fn default() -> Self {
    State::Nominal(StateSpecificOperator {
      _state_discriminant: StateDiscriminantNominal {},
    })
  }
}
