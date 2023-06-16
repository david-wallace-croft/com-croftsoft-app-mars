// =============================================================================
//! - Tank Operater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-04
//! - Updated: 2023-06-16
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::ai::state_space_node::StateSpaceNode;

pub mod default;

pub trait TankOperator {
  fn fire(&mut self);

  fn get_id(&self) -> usize;

  // TODO: was iterator
  fn get_path(&self) -> Vec<StateSpaceNode>;

  fn update(
    &mut self,
    time_delta: f64,
  );
}
