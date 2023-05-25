// =============================================================================
//! - Tank Operater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-04
//! - Updated: 2023-05-24
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::tank::default::DefaultTank;
use crate::ai::state_space_node::StateSpaceNode;
use crate::ai::tank_console::TankConsole;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub mod default;
pub mod updater;

pub trait TankOperator {
  fn fire(&mut self);

  fn get_id(&self) -> usize;

  // TODO: was iterator
  fn get_path(&self) -> Vec<StateSpaceNode>;

  fn set_tank_console(
    &mut self,
    tank_console: Rc<RefCell<dyn TankConsole>>,
  );

  fn update(
    &mut self,
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<DefaultTank>>>>>,
    time_delta: f64,
  );
}
