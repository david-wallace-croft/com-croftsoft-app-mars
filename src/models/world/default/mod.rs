// =============================================================================
//! - World for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-29
//! - Updated: 2023-05-31
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::World;
use crate::models::ammo_dump::AmmoDump;
use crate::models::bullet::Bullet;
use crate::models::explosion::Explosion;
use crate::models::obstacle::default::DefaultObstacle;
use crate::models::tank::default::DefaultTank;
use crate::models::tank::Tank;
use crate::models::tank_operator::TankOperator;
use com_croftsoft_core::math::geom::circle::CircleAccessor;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Default)]
pub struct DefaultWorld {
  ammo_dumps: Rc<RefCell<VecDeque<Box<dyn AmmoDump>>>>,
  bullets: Rc<RefCell<VecDeque<Box<dyn Bullet>>>>,
  explosions: Rc<RefCell<VecDeque<Box<dyn Explosion>>>>,
  obstacles: Rc<RefCell<VecDeque<DefaultObstacle>>>,
  tank_operators: Rc<RefCell<VecDeque<Rc<RefCell<dyn TankOperator>>>>>,
  tanks: Rc<RefCell<VecDeque<Rc<RefCell<dyn Tank>>>>>,
}

impl World for DefaultWorld {
  fn add_ammo_dump(
    &self,
    ammo_dump: Box<dyn AmmoDump>,
  ) {
    self.ammo_dumps.borrow_mut().push_back(ammo_dump);
  }

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

  fn add_obstacle(
    &self,
    obstacle: DefaultObstacle,
  ) {
    self.obstacles.borrow_mut().push_back(obstacle);
  }

  fn add_tank(
    &self,
    tank: Rc<RefCell<DefaultTank>>,
  ) {
    self.tanks.borrow_mut().push_back(tank);
  }

  fn add_tank_operator(
    &self,
    tank_operator: Rc<RefCell<dyn TankOperator>>,
  ) {
    self.tank_operators.borrow_mut().push_back(tank_operator);
  }

  fn compute_bullet_damage(
    &self,
    circle: &dyn CircleAccessor,
  ) -> f64 {
    self
      .bullets
      .borrow_mut()
      .iter_mut()
      .filter(|bullet| bullet.intersects_circle(circle))
      .fold(0., |damage, bullet| {
        let updated_damage: f64 = damage + bullet.get_damage();
        bullet.mark_spent();
        return updated_damage;
      })
  }

  fn compute_explosion_damage(
    &self,
    circle: &dyn CircleAccessor,
  ) -> f64 {
    self
      .explosions
      .borrow()
      .iter()
      .filter(|explosion| explosion.intersects_circle(circle))
      .fold(0., |damage, explosion| damage + explosion.get_damage())
  }

  fn get_ammo_dumps(&self) -> Rc<RefCell<VecDeque<Box<dyn AmmoDump>>>> {
    self.ammo_dumps.clone()
  }

  fn get_bullets(&self) -> Rc<RefCell<VecDeque<Box<dyn Bullet>>>> {
    self.bullets.clone()
  }

  fn get_explosions(&self) -> Rc<RefCell<VecDeque<Box<dyn Explosion>>>> {
    self.explosions.clone()
  }

  fn get_obstacles(&self) -> Rc<RefCell<VecDeque<DefaultObstacle>>> {
    self.obstacles.clone()
  }

  fn get_tank_operators(
    &self
  ) -> Rc<RefCell<VecDeque<Rc<RefCell<dyn TankOperator>>>>> {
    self.tank_operators.clone()
  }

  fn get_tanks(&self) -> Rc<RefCell<VecDeque<Rc<RefCell<dyn Tank>>>>> {
    self.tanks.clone()
  }

  fn is_blocked_by_ammo_dump(
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

  fn is_blocked_by_tank(
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
