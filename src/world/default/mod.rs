// =============================================================================
//! - World for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-29
//! - Updated: 2023-06-28
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::World;
use crate::ai::tank_operator::TankOperator;
use crate::model::ammo_dump::AmmoDump;
use crate::model::bullet::Bullet;
use crate::model::explosion::Explosion;
use crate::model::obstacle::Obstacle;
use crate::visitor::{Visitor, VisitorAcceptor};
use com_croftsoft_core::math::geom::circle::CircleAccessor;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Default)]
pub struct DefaultWorld {
  ammo_dumps: Rc<RefCell<VecDeque<Box<dyn AmmoDump>>>>,
  bullets: Rc<RefCell<VecDeque<Box<dyn Bullet>>>>,
  explosions: Rc<RefCell<VecDeque<Box<dyn Explosion>>>>,
  obstacles: Rc<RefCell<VecDeque<Box<dyn Obstacle>>>>,
  tank_operators: Rc<RefCell<VecDeque<Box<dyn TankOperator>>>>,
}

impl VisitorAcceptor for DefaultWorld {
  fn accept_visitor(
    &self,
    visitor: &dyn Visitor,
  ) {
    for ammo_dump in self.ammo_dumps.borrow_mut().iter_mut() {
      visitor.visit_ammo_dump(ammo_dump.as_mut());
    }
    for obstacle in self.obstacles.borrow_mut().iter_mut() {
      visitor.visit_obstacle(obstacle.as_mut());
    }
    for tank_operator in self.tank_operators.borrow().iter() {
      let tank = tank_operator.get_tank();
      visitor.visit_tank(&mut *tank.borrow_mut());
    }
  }
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
    obstacle: Box<dyn Obstacle>,
  ) {
    self.obstacles.borrow_mut().push_back(obstacle);
  }

  fn add_tank_operator(
    &self,
    tank_operator: Box<dyn TankOperator>,
  ) {
    self.tank_operators.borrow_mut().push_back(tank_operator);
  }

  fn clear(&self) {
    self.ammo_dumps.borrow_mut().clear();
    self.bullets.borrow_mut().clear();
    self.explosions.borrow_mut().clear();
    self.obstacles.borrow_mut().clear();
    self.tank_operators.borrow_mut().clear();
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

  fn get_obstacles(&self) -> Rc<RefCell<VecDeque<Box<dyn Obstacle>>>> {
    self.obstacles.clone()
  }

  fn get_tank_operators(&self) -> Rc<RefCell<VecDeque<Box<dyn TankOperator>>>> {
    self.tank_operators.clone()
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
      if circle.intersects_circle(&obstacle.get_circle()) {
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
    for tank_operator in self.tank_operators.borrow().iter() {
      let tank = tank_operator.get_tank();
      if tank.borrow().is_active() && tank.borrow().intersects_circle(circle) {
        return true;
      }
    }
    false
  }
}
