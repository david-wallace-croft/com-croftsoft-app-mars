// =============================================================================
//! - Obstacle Preparer for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-13
//! - Updated: 2023-05-20
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::state::ObstacleState;
use crate::models::world::default::DefaultWorld;
use com_croftsoft_lib_role::Preparer;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct ObstaclePreparer {
  obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
}

impl ObstaclePreparer {
  pub fn new(world: Rc<RefCell<DefaultWorld>>) -> Self {
    let obstacles = world.borrow().obstacles.clone();
    Self {
      obstacles,
    }
  }
}

impl Preparer for ObstaclePreparer {
  fn prepare(&mut self) {
    let length: usize = self.obstacles.borrow().len();
    for _index in 0..length {
      let mut obstacle = self.obstacles.borrow_mut().pop_front().unwrap();
      obstacle.prepare();
      self.obstacles.borrow_mut().push_back(obstacle);
    }
  }
}
