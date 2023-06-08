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
pub struct State<S> {
  _state_discriminant: S,
}

impl State<StateDiscriminantCooling> {
  pub fn reset(self) -> State<StateDiscriminantNominal> {
    State {
      _state_discriminant: StateDiscriminantNominal {},
    }
  }
}

impl State<StateDiscriminantExploding> {
  pub fn cool(self) -> State<StateDiscriminantCooling> {
    State {
      _state_discriminant: StateDiscriminantCooling {},
    }
  }
}

impl State<StateDiscriminantNominal> {
  pub fn explode(self) -> State<StateDiscriminantExploding> {
    State {
      _state_discriminant: StateDiscriminantExploding {},
    }
  }
}

pub enum StateMachine {
  Cooling(State<StateDiscriminantCooling>),
  Exploding(State<StateDiscriminantExploding>),
  Nominal(State<StateDiscriminantNominal>),
}

impl Default for StateMachine {
  fn default() -> Self {
    StateMachine::Nominal(State {
      _state_discriminant: StateDiscriminantNominal {},
    })
  }
}
