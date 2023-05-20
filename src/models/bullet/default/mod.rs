// =============================================================================
//! - Default Bullet for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-10
//! - Updated: 2023-05-20
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::{Bullet, BulletAccessor};
use crate::constants::{
  BULLET_DAMAGE, BULLET_RADIUS, BULLET_RANGE, BULLET_VELOCITY, BULLET_Z,
};
use crate::engine::traits::{Damageable, Model, ModelAccessor};
use crate::models::world::World;
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use com_croftsoft_lib_role::Preparer;
use core::cell::RefCell;
use std::rc::Rc;

pub struct DefaultBullet {
  active: bool,
  circle: Circle,
  distance: f64,
  heading: f64,
  id: usize,
  origin_x: f64,
  origin_y: f64,
  updated: bool,
  world: Rc<RefCell<dyn World>>,
}

impl DefaultBullet {
  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn new(
    heading: f64,
    id: usize,
    origin_x: f64,
    origin_y: f64,
    world: Rc<RefCell<dyn World>>,
  ) -> Self {
    let mut bullet = Self {
      active: false,
      circle: Circle {
        center_x: 0.,
        center_y: 0.,
        radius: BULLET_RADIUS,
      },
      distance: 0.,
      heading: 0.,
      id,
      origin_x: 0.,
      origin_y: 0.,
      updated: false,
      world,
    };
    bullet.fire(heading, origin_x, origin_y);
    bullet
  }
}

impl Bullet for DefaultBullet {
  fn fire(
    &mut self,
    heading: f64,
    origin_x: f64,
    origin_y: f64,
  ) {
    self.active = true;
    self.circle.set_center(origin_x, origin_y);
    self.distance = 0.;
    self.heading = heading;
    self.origin_x = origin_x;
    self.origin_y = origin_y;
    self.updated = true;
  }
}

impl BulletAccessor for DefaultBullet {}

impl Model for DefaultBullet {
  fn set_center(
    &mut self,
    x: f64,
    y: f64,
  ) {
    self.circle.set_center(x, y);
  }

  fn update(
    &mut self,
    time_delta: f64,
  ) {
    if !self.active {
      return;
    }
    self.updated = true;
    self.distance += time_delta * BULLET_VELOCITY;
    if self.distance > BULLET_RANGE || self.distance < 0. {
      self.active = false;
      return;
    }
    let center_x = self.origin_x + self.distance * self.heading.cos();
    let center_y = self.origin_y + self.distance * self.heading.sin();
    self.circle.set_center(center_x, center_y);
    // TODO: old code fetched first damageable or impassable at point from World
    let world = self.world.borrow();
    let obstacles = world.get_obstacles();
    for obstacle in obstacles.borrow_mut().iter_mut() {
      if obstacle.contains(center_x, center_y) {
        self.active = false;
        obstacle.add_damage(BULLET_DAMAGE);
        return;
      }
    }
    let tanks = world.get_tanks();
    for tank in tanks.borrow_mut().iter_mut() {
      let mut tank = tank.borrow_mut();
      if tank.contains(center_x, center_y) {
        self.active = false;
        tank.add_damage(BULLET_DAMAGE);
        return;
      }
    }
    let ammo_dumps = world.get_ammo_dumps();
    for ammo_dump in ammo_dumps.borrow_mut().iter_mut() {
      if ammo_dump.contains(center_x, center_y) {
        self.active = false;
        ammo_dump.add_damage(BULLET_DAMAGE);
        return;
      }
    }
  }
}

impl ModelAccessor for DefaultBullet {
  fn contains(
    &self,
    x: f64,
    y: f64,
  ) -> bool {
    self.circle.contains(x, y)
  }

  fn get_circle(&self) -> Circle {
    self.circle
  }

  fn get_z(&self) -> f64 {
    BULLET_Z
  }

  fn intersects_circle(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool {
    self.circle.intersects_circle(circle)
  }

  fn is_active(&self) -> bool {
    self.active
  }

  fn is_updated(&self) -> bool {
    self.updated
  }
}

impl Preparer for DefaultBullet {
  fn prepare(&mut self) {
    self.updated = false;
  }
}
