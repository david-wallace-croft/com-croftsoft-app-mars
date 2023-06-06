// =============================================================================
//! - Default Ammo Dump State for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-05
//! - Updated: 2023-06-06
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

pub enum DefaultAmmoDumpStateEvent {
  Cool,
  Explode,
  Reset,
}

// TODO: Do we really need Copy here?
#[derive(Clone, Copy)]
pub enum DefaultAmmoDumpStateMachine {
  Cooling(DefaultAmmoDumpState<Cooling>),
  Exploding(DefaultAmmoDumpState<Exploding>),
  Nominal(DefaultAmmoDumpState<Nominal>),
}

impl DefaultAmmoDumpStateMachine {
  pub fn is_cooling(&self) -> bool {
    matches!(self, DefaultAmmoDumpStateMachine::Cooling(_))
  }

  pub fn is_exploding(&self) -> bool {
    matches!(self, DefaultAmmoDumpStateMachine::Exploding(_))
  }

  pub fn is_nominal(&self) -> bool {
    matches!(self, DefaultAmmoDumpStateMachine::Nominal(_))
  }

  pub fn transition(
    self,
    event: DefaultAmmoDumpStateEvent,
  ) -> Self {
    match (self, event) {
      (
        DefaultAmmoDumpStateMachine::Nominal(state),
        DefaultAmmoDumpStateEvent::Explode,
      ) => DefaultAmmoDumpStateMachine::Exploding(state.explode()),
      (
        DefaultAmmoDumpStateMachine::Exploding(state),
        DefaultAmmoDumpStateEvent::Cool,
      ) => DefaultAmmoDumpStateMachine::Cooling(state.cool()),
      (
        DefaultAmmoDumpStateMachine::Cooling(state),
        DefaultAmmoDumpStateEvent::Reset,
      ) => DefaultAmmoDumpStateMachine::Nominal(state.reset()),
      _ => self,
    }
  }
}

impl Default for DefaultAmmoDumpStateMachine {
  fn default() -> Self {
    DefaultAmmoDumpStateMachine::Nominal(DefaultAmmoDumpState {
      _state: Nominal {},
    })
  }
}
