// =============================================================================
//! - Obstacle Preparer for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-13
//! - Updated: 2023-05-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::state::ObstacleState;
use crate::models::world::World;
use com_croftsoft_lib_role::Preparer;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct ObstaclePreparer {
  obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
}

impl ObstaclePreparer {
  pub fn new(world: Rc<RefCell<World>>) -> Self {
    let obstacles = world.borrow().obstacles.clone();
    Self {
      obstacles,
    }
  }
}

impl Preparer for ObstaclePreparer {
  fn prepare(&mut self) {
    self
      .obstacles
      .borrow_mut()
      .iter_mut()
      .for_each(|obstacle| obstacle.prepare());
  }
}
