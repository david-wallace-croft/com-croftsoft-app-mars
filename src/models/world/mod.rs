// =============================================================================
//! - World for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-29
//! - Updated: 2023-05-12
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::bullet::Bullet;
use crate::ai::tank_operator::TankOperator;
use crate::engine::traits::ModelAccessor;
use crate::models::ammo_dump::default::DefaultAmmoDump;
use crate::models::obstacle::state::ObstacleState;
use crate::models::tank::state::TankState;
use com_croftsoft_core::math::geom::circle::CircleAccessor;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub mod builder;
pub mod director;
pub mod seed;
pub mod updater;

#[derive(Default)]
pub struct World {
  pub ammo_dumps: Rc<RefCell<VecDeque<DefaultAmmoDump>>>,
  pub bullets: Rc<RefCell<Vec<Box<dyn Bullet>>>>,
  pub obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
  pub tank_operators: Rc<RefCell<VecDeque<Rc<RefCell<dyn TankOperator>>>>>,
  pub tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
}

impl World {
  // TODO: argument was Model in old code; could be Shape
  pub fn is_blocked_by_impassable(
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

  pub fn is_blocked_by_ammo_dump(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool {
    for ammo_dump in self.ammo_dumps.borrow().iter() {
      // TODO: use a function to determine if there is one
      if ammo_dump.intersects_circle(circle) {
        return true;
      }
    }
    false
  }

  pub fn is_blocked_by_tank(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool {
    // TODO: use a function to determine if there is one
    for tank in self.tanks.borrow().iter() {
      if tank.borrow().intersects_circle(circle) {
        return true;
      }
    }
    false
  }
}
