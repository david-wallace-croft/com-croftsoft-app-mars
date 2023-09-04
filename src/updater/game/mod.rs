// =============================================================================
//! - Game Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-07-09
//! - Updated: 2023-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::root::{Root, RootMutator};
use com_croftsoft_lib_role::Updater;
use std::rc::Rc;

pub struct GameUpdater {
  root: Rc<dyn Root>,
  root_mutator: Rc<dyn RootMutator>,
}

impl GameUpdater {
  pub fn new(
    root: Rc<dyn Root>,
    root_mutator: Rc<dyn RootMutator>,
  ) -> Self {
    Self {
      root,
      root_mutator,
    }
  }
}

impl Updater for GameUpdater {
  fn update(&self) {
    let obstacles = self.root.get_world().get_obstacles();
    if obstacles.borrow().len() == 0 {
      self.root_mutator.get_game_mutator().increment_level();
      self.root.get_inputs().borrow_mut().reset_requested = true;
      return;
    }
    let tank_operators = self.root.get_world().get_tank_operators();
    if tank_operators.borrow().len() != 0 {
      return;
    }
    self.root.get_inputs().borrow_mut().reset_requested = true;
  }
}
