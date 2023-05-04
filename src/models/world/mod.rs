// =============================================================================
//! - World for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-29
//! - Updated: 2023-05-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub mod builder;
pub mod director;

use crate::engine::traits::ModelAccessor;
use crate::models::ammo_dump::default::DefaultAmmoDump;
use crate::models::obstacle::state::ObstacleState;
use crate::models::tank::state::TankState;
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Default)]
pub struct World {
  pub ammo_dumps: Rc<RefCell<VecDeque<DefaultAmmoDump>>>,
  pub obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
  pub tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
}

impl World {
  // TODO: argument was Model in old code; could be Shape
  pub fn is_blocked(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool {
    // TODO: Use CollisionDetector
    // TODO: Old code iterated over array of Impassable
    for obstacle in self.obstacles.borrow().iter() {
      if circle.intersects_circle(&obstacle.circle) {
        return true;
      }
    }
    self.is_blocked_by_tank(circle)
  }

  pub fn is_blocked_by_tank(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool {
    let mut tank_circle = Circle::default();
    for tank in self.tanks.borrow().iter() {
      tank_circle = tank.borrow().get_shape(tank_circle);
      if circle.intersects_circle(&tank_circle) {
        return true;
      }
    }
    false
  }
}
