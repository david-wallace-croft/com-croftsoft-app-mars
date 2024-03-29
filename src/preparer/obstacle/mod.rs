// =============================================================================
//! - Obstacle Preparer for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-13
//! - Updated: 2023-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::model::obstacle::Obstacle;
use com_croftsoft_lib_role::Preparer;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct ObstaclePreparer {
  obstacles: Rc<RefCell<VecDeque<Box<dyn Obstacle>>>>,
}

impl ObstaclePreparer {
  pub fn new(obstacles: Rc<RefCell<VecDeque<Box<dyn Obstacle>>>>) -> Self {
    Self {
      obstacles,
    }
  }
}

impl Preparer for ObstaclePreparer {
  fn prepare(&self) {
    let length: usize = self.obstacles.borrow().len();
    for _index in 0..length {
      let mut obstacle = self.obstacles.borrow_mut().pop_front().unwrap();
      obstacle.prepare();
      self.obstacles.borrow_mut().push_back(obstacle);
    }
  }
}
