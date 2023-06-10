// =============================================================================
//! - Default Explosion State for CroftSoft Mars
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
pub struct FromExploding;
#[derive(Clone, Copy)]
pub struct FromFading;

#[derive(Clone, Copy)]
pub enum State {
  Exploding(Transition<FromExploding>),
  Fading(Transition<FromFading>),
  Inactive,
}

impl Default for State {
  fn default() -> Self {
    State::Exploding(Transition {
      from: PhantomData,
    })
  }
}

#[derive(Clone, Copy)]
pub struct Transition<F> {
  from: PhantomData<F>,
}

impl Transition<FromExploding> {
  pub fn to_fading(self) -> State {
    State::Fading(Transition {
      from: PhantomData,
    })
  }
}

impl Transition<FromFading> {
  pub fn to_inactive(self) -> State {
    State::Inactive
  }
}
