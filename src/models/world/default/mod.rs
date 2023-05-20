// =============================================================================
//! - World for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-29
//! - Updated: 2023-05-20
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::factory::WorldFactory;
use super::World;
use crate::engine::traits::ModelAccessor;
use crate::models::ammo_dump::default::DefaultAmmoDump;
use crate::models::bullet::Bullet;
use crate::models::explosion::Explosion;
use crate::models::obstacle::state::ObstacleState;
use crate::models::tank::state::TankState;
use crate::models::tank_operator::TankOperator;
use com_croftsoft_core::math::geom::circle::CircleAccessor;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct DefaultWorld {
  pub ammo_dumps: Rc<RefCell<VecDeque<DefaultAmmoDump>>>,
  bullets: Rc<RefCell<VecDeque<Box<dyn Bullet>>>>,
  pub explosions: Rc<RefCell<VecDeque<Box<dyn Explosion>>>>,
  // TODO: move factory out of World
  pub factory: Rc<dyn WorldFactory>,
  pub obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
  pub tank_operators: Rc<RefCell<VecDeque<Rc<RefCell<dyn TankOperator>>>>>,
  pub tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
}

impl DefaultWorld {
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

  pub fn new(factory: Rc<dyn WorldFactory>) -> Self {
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

impl World for DefaultWorld {
  fn add_bullet(
    &self,
    bullet: Box<dyn Bullet>,
  ) {
    self.bullets.borrow_mut().push_back(bullet);
  }

  fn add_explosion(
    &self,
    explosion: Box<dyn Explosion>,
  ) {
    self.explosions.borrow_mut().push_back(explosion);
  }

  fn get_ammo_dumps(&self) -> Rc<RefCell<VecDeque<DefaultAmmoDump>>> {
    self.ammo_dumps.clone()
  }

  fn get_bullets(&self) -> Rc<RefCell<VecDeque<Box<dyn Bullet>>>> {
    self.bullets.clone()
  }

  fn get_factory(&self) -> Rc<dyn WorldFactory> {
    self.factory.clone()
  }

  fn get_obstacles(&self) -> Rc<RefCell<VecDeque<ObstacleState>>> {
    self.obstacles.clone()
  }

  fn get_tanks(&self) -> Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>> {
    self.tanks.clone()
  }

  // TODO: argument was Model in old code; could be Shape
  fn is_blocked_by_impassable(
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
}
