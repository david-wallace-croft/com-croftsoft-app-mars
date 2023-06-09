// =============================================================================
//! - Tank Operater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-04
//! - Updated: 2023-06-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::ai::state_space_node::StateSpaceNode;
use crate::model::tank::Tank;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub mod default;

pub trait TankOperator {
  fn fire(&mut self);

  fn get_id(&self) -> usize;

  fn get_nodes(&self) -> Vec<StateSpaceNode>;

  // TODO: was iterator
  fn get_path(&self) -> VecDeque<StateSpaceNode>;

  fn get_tank(&self) -> Rc<RefCell<dyn Tank>>;

  fn update(
    &mut self,
    time_delta: f64,
  );
}
