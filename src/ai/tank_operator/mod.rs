// =============================================================================
//! - Tank Operater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-04
//! - Updated: 2023-05-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::state_space_node::StateSpaceNode;
use super::tank_console::TankConsole;
use crate::models::tank::state::TankState;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub mod default;

#[derive(Default)]
pub struct TankOperatorState {
  // tank_console: Option<Rc<RefCell<dyn TankConsole>>>,
}

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
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
    time_delta: f64,
  );
}
