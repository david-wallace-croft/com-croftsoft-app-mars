// =============================================================================
//! - World for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-29
//! - Updated: 2023-05-17
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use self::factory::WorldFactory;
use super::bullet::Bullet;
use super::explosion::Explosion;
use crate::ai::tank_operator::TankOperator;
use crate::engine::traits::ModelAccessor;
use crate::models::ammo_dump::default::DefaultAmmoDump;
use crate::models::obstacle::state::ObstacleState;
use crate::models::tank::state::TankState;
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub mod builder;
pub mod director;
pub mod factory;
pub mod preparer;
pub mod seed;
pub mod updater;

pub struct World {
  pub ammo_dumps: Rc<RefCell<VecDeque<DefaultAmmoDump>>>,
  pub bullets: Rc<RefCell<VecDeque<Box<dyn Bullet>>>>,
  pub explosions: Rc<RefCell<VecDeque<Box<dyn Explosion>>>>,
  pub factory: Rc<RefCell<dyn WorldFactory>>,
  pub obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
  pub tank_operators: Rc<RefCell<VecDeque<Rc<RefCell<dyn TankOperator>>>>>,
  pub tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
}

impl World {
  pub fn add_explosion(
    &self,
    circle: Circle,
    damage: f64,
  ) {
    let explosion = self.factory.borrow().make_explosion(circle, damage);
    self.explosions.borrow_mut().push_back(explosion);
  }

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

  pub fn new(factory: Rc<RefCell<dyn WorldFactory>>) -> Self {
    Self {
      ammo_dumps: Default::default(),
      bullets: Default::default(),
      explosions: Default::default(),
      factory,
      obstacles: Default::default(),
      tank_operators: Default::default(),
      tanks: Default::default(),
    }
  }
}
