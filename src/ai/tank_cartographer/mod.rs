// =============================================================================
//! - Tank Cartographer for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-07
//! - Updated: 2023-04-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::ai::tank_console::TankConsole;
use std::cell::RefCell;
use std::rc::Rc;
use super::state_space_node::StateSpaceNode;

pub struct TankCartographer {
  adjacent_list: Vec<()>,
  directions: usize,
  goal_state_space_node: StateSpaceNode,
  init_step_size: f64,
  start_state_space_node: StateSpaceNode,
  tank_console: Rc<RefCell<dyn TankConsole>>,
}
