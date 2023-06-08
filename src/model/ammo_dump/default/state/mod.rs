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
pub struct DefaultAmmoDumpState<S> {
  _state_discriminant: S,
}

impl DefaultAmmoDumpState<StateDiscriminantCooling> {
  pub fn reset(self) -> DefaultAmmoDumpState<StateDiscriminantNominal> {
    DefaultAmmoDumpState {
      _state_discriminant: StateDiscriminantNominal {},
    }
  }
}

impl DefaultAmmoDumpState<StateDiscriminantExploding> {
  pub fn cool(self) -> DefaultAmmoDumpState<StateDiscriminantCooling> {
    DefaultAmmoDumpState {
      _state_discriminant: StateDiscriminantCooling {},
    }
  }
}

impl DefaultAmmoDumpState<StateDiscriminantNominal> {
  pub fn explode(self) -> DefaultAmmoDumpState<StateDiscriminantExploding> {
    DefaultAmmoDumpState {
      _state_discriminant: StateDiscriminantExploding {},
    }
  }
}

pub enum DefaultAmmoDumpStateMachine {
  Cooling(DefaultAmmoDumpState<StateDiscriminantCooling>),
  Exploding(DefaultAmmoDumpState<StateDiscriminantExploding>),
  Nominal(DefaultAmmoDumpState<StateDiscriminantNominal>),
}

impl Default for DefaultAmmoDumpStateMachine {
  fn default() -> Self {
    DefaultAmmoDumpStateMachine::Nominal(DefaultAmmoDumpState {
      _state_discriminant: StateDiscriminantNominal {},
    })
  }
}
