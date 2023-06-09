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

use core::marker::PhantomData;

pub struct CoolingState;
pub struct ExplodingState;
pub struct NominalState;

pub struct FromState<D> {
  phantom: PhantomData<D>,
}

impl FromState<CoolingState> {
  pub fn to_nominal(&self) -> State {
    State::Nominal(FromState {
      phantom: PhantomData,
    })
  }
}

impl FromState<ExplodingState> {
  pub fn to_cooling(&self) -> State {
    State::Cooling(FromState {
      phantom: PhantomData,
    })
  }
}

impl FromState<NominalState> {
  pub fn to_exploding(&self) -> State {
    State::Exploding(FromState {
      phantom: PhantomData,
    })
  }
}

pub enum State {
  Cooling(FromState<CoolingState>),
  Exploding(FromState<ExplodingState>),
  Nominal(FromState<NominalState>),
}

impl Default for State {
  fn default() -> Self {
    State::Nominal(FromState {
      phantom: PhantomData,
    })
  }
}
