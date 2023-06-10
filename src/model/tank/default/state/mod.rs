// =============================================================================
//! - Default Tank State for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-10
//! - Updated: 2023-06-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use core::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct FromBurning;
#[derive(Clone, Copy)]
pub struct FromNominal;
#[derive(Clone, Copy)]
pub struct FromSparking;

#[derive(Clone, Copy)]
pub enum State {
  Burning(Transition<FromBurning>),
  Inactive,
  Nominal(Transition<FromNominal>),
  Sparking(Transition<FromSparking>),
}

impl Default for State {
  fn default() -> Self {
    State::Nominal(Transition {
      from: PhantomData,
    })
  }
}

#[derive(Clone, Copy)]
pub struct Transition<F> {
  from: PhantomData<F>,
}

impl Transition<FromBurning> {
  pub fn to_inactive(self) -> State {
    State::Inactive
  }
}

impl Transition<FromNominal> {
  pub fn to_burning(self) -> State {
    State::Burning(Transition {
      from: PhantomData,
    })
  }

  pub fn to_sparking(self) -> State {
    State::Sparking(Transition {
      from: PhantomData,
    })
  }
}

impl Transition<FromSparking> {
  pub fn to_burning(self) -> State {
    State::Burning(Transition {
      from: PhantomData,
    })
  }

  pub fn to_nominal(self) -> State {
    State::Nominal(Transition {
      from: PhantomData,
    })
  }
}
