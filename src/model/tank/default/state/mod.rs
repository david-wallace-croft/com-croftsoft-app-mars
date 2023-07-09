// =============================================================================
//! - Default Tank State for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-10
//! - Updated: 2023-07-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use core::marker::PhantomData;

pub struct DataBurning;
pub struct DataNominal;
pub struct DataSparking;

pub enum State {
  Burning(StateOperator<DataBurning>),
  Inactive,
  Nominal(StateOperator<DataNominal>),
  Sparking(StateOperator<DataSparking>),
}

impl Default for State {
  fn default() -> Self {
    State::Nominal(StateOperator {
      _data: PhantomData,
    })
  }
}

#[derive(Clone, Copy)]
pub struct StateOperator<F> {
  _data: PhantomData<F>,
}

impl StateOperator<DataBurning> {
  // TODO: figure out how to move self during a state transition so cannot reuse
  pub fn to_inactive(&self) -> State {
    State::Inactive
  }
}

impl StateOperator<DataNominal> {
  pub fn to_burning(&self) -> State {
    State::Burning(StateOperator {
      _data: PhantomData,
    })
  }

  pub fn to_sparking(&self) -> State {
    State::Sparking(StateOperator {
      _data: PhantomData,
    })
  }
}

impl StateOperator<DataSparking> {
  pub fn to_burning(&self) -> State {
    State::Burning(StateOperator {
      _data: PhantomData,
    })
  }

  pub fn to_nominal(&self) -> State {
    State::Nominal(StateOperator {
      _data: PhantomData,
    })
  }
}
