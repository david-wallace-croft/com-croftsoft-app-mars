// =============================================================================
//! - Default Explosion State for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-10
//! - Updated: 2023-06-14
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use core::marker::PhantomData;

pub struct DataExploding;
pub struct DataFading;

pub enum State {
  Exploding(StateOperator<DataExploding>),
  Fading(StateOperator<DataFading>),
  Inactive,
}

impl Default for State {
  fn default() -> Self {
    State::Exploding(StateOperator {
      _data: PhantomData,
    })
  }
}

pub struct StateOperator<D> {
  _data: PhantomData<D>,
}

impl StateOperator<DataExploding> {
  pub fn to_fading(&self) -> State {
    State::Fading(StateOperator {
      _data: PhantomData,
    })
  }
}

impl StateOperator<DataFading> {
  pub fn to_inactive(&self) -> State {
    State::Inactive
  }
}
