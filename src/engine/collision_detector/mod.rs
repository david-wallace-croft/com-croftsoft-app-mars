// =============================================================================
//! - Collision Detector for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-03-18
//! - Updated: 2023-03-23
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::state::obstacle::ObstacleState;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct CollisionDetector {
  pub obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
}

impl CollisionDetector {
  pub fn new(obstacles: Rc<RefCell<VecDeque<ObstacleState>>>) -> Self {
    Self {
      obstacles,
    }
  }
}

impl CollisionDetector {
  pub fn detect_collision(
    &self,
    circle_accessor: &dyn com_croftsoft_core::math::geom::circle::CircleAccessor,
  ) -> bool {
    for obstacle in self.obstacles.borrow().iter() {
      if circle_accessor.intersects_circle(&obstacle.circle) {
        return true;
      }
    }
    false
  }
}
