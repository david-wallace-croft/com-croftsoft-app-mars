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

pub struct FromCooling;
pub struct FromExploding;
pub struct FromNominal;

pub struct Transition<F> {
  from: PhantomData<F>,
}

impl Transition<FromCooling> {
  pub fn to_nominal(&self) -> State {
    State::Nominal(Transition {
      from: PhantomData,
    })
  }
}

impl Transition<FromExploding> {
  pub fn to_cooling(&self) -> State {
    State::Cooling(Transition {
      from: PhantomData,
    })
  }
}

impl Transition<FromNominal> {
  pub fn to_exploding(&self) -> State {
    State::Exploding(Transition {
      from: PhantomData,
    })
  }
}

pub enum State {
  Cooling(Transition<FromCooling>),
  Exploding(Transition<FromExploding>),
  Nominal(Transition<FromNominal>),
}

impl Default for State {
  fn default() -> Self {
    State::Nominal(Transition {
      from: PhantomData,
    })
  }
}
