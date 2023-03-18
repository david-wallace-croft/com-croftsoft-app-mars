// =============================================================================
//! - Collision Detector for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-03-18
//! - Updated: 2023-03-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::Entity;
use crate::state::obstacle::Obstacle;
use core::cell::RefCell;
use std::rc::Rc;

pub struct CollisionDetector {
  pub obstacles: Rc<RefCell<Vec<Obstacle>>>,
}

impl CollisionDetector {
  pub fn new(obstacles: Rc<RefCell<Vec<Obstacle>>>) -> Self {
    Self {
      obstacles,
    }
  }
}

impl CollisionDetector {
  pub fn detect_collision(
    &self,
    circle_accessor: &dyn com_croftsoft_core::math::geom::circle::CircleAccessor,
    uuid: &str,
  ) -> bool {
    for obstacle in self.obstacles.borrow().iter() {
      if obstacle.has_uuid(uuid) {
        continue;
      }
      if circle_accessor.intersects_circle(&obstacle.circle) {
        return true;
      }
    }
    false
  }
}
