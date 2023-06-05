// =============================================================================
//! - Default Ammo Dump State for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-05
//! - Updated: 2023-06-05
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[derive(Clone, Copy)]
struct Cooling;

#[derive(Clone, Copy)]
struct Exploding;

#[derive(Clone, Copy)]
struct Nominal;

#[derive(Clone, Copy)]
struct DefaultAmmoDumpState<S> {
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

enum DefaultAmmoDumpStateEvent {
  Cool,
  Explode,
  Reset,
}

#[derive(Clone, Copy)]
enum DefaultAmmoDumpStateMachine {
  Cooling(DefaultAmmoDumpState<Cooling>),
  Exploding(DefaultAmmoDumpState<Exploding>),
  Nominal(DefaultAmmoDumpState<Nominal>),
}

impl DefaultAmmoDumpStateMachine {
  fn transition(
    self,
    event: DefaultAmmoDumpStateEvent,
  ) -> Self {
    match (self, event) {
      (
        DefaultAmmoDumpStateMachine::Nominal(state),
        DefaultAmmoDumpStateEvent::Explode,
      ) => DefaultAmmoDumpStateMachine::Exploding(state.explode()),
      _ => self,
    }
  }
}