// =============================================================================
//! - Default Ammo Dump State for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-05
//! - Updated: 2023-06-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[derive(Clone, Copy)]
pub struct Cooling;

#[derive(Clone, Copy)]
pub struct Exploding;

#[derive(Clone, Copy)]
pub struct Nominal;

#[derive(Clone, Copy)]
pub struct DefaultAmmoDumpState<S> {
  _state: S,
}

impl DefaultAmmoDumpState<Cooling> {
  pub fn reset(self) -> DefaultAmmoDumpState<Nominal> {
    DefaultAmmoDumpState {
      _state: Nominal {},
    }
  }
}

impl DefaultAmmoDumpState<Exploding> {
  pub fn cool(self) -> DefaultAmmoDumpState<Cooling> {
    DefaultAmmoDumpState {
      _state: Cooling {},
    }
  }
}

impl DefaultAmmoDumpState<Nominal> {
  pub fn explode(self) -> DefaultAmmoDumpState<Exploding> {
    DefaultAmmoDumpState {
      _state: Exploding {},
    }
  }
}

// TODO: Do we really need Copy here?
#[derive(Clone, Copy)]
pub enum DefaultAmmoDumpStateMachine {
  Cooling(DefaultAmmoDumpState<Cooling>),
  Exploding(DefaultAmmoDumpState<Exploding>),
  Nominal(DefaultAmmoDumpState<Nominal>),
}

impl Default for DefaultAmmoDumpStateMachine {
  fn default() -> Self {
    DefaultAmmoDumpStateMachine::Nominal(DefaultAmmoDumpState {
      _state: Nominal {},
    })
  }
}
