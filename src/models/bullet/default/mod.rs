// =============================================================================
//! - Default Bullet for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-10
//! - Updated: 2023-05-11
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_core::math::geom::circle::Circle;

use crate::constants::{
  BULLET_RADIUS, BULLET_RANGE, BULLET_VELOCITY, BULLET_Z,
};
use crate::engine::traits::{Model, ModelAccessor};
use crate::models::world::World;
use core::cell::RefCell;
use std::rc::Rc;

use super::{Bullet, BulletAccessor};

pub struct DefaultBullet {
  active: bool,
  circle: Circle,
  distance: f64,
  heading: f64,
  id: usize,
  origin_x: f64,
  origin_y: f64,
  updated: bool,
  world: Rc<RefCell<World>>,
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
    world: Rc<RefCell<World>>,
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
  fn prepare(&mut self) {
    self.updated = false;
  }

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
    let mut obstacles = world.obstacles.borrow_mut();
    for obstacle in obstacles.iter_mut() {
      if obstacle.contains(center_x, center_y) {
        // TODO: left off here
        todo!();
      }
    }
    todo!()
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

  fn get_shape(
    &self,
    mut circle: Circle,
  ) -> Circle {
    circle.center_x = self.circle.center_x;
    circle.center_y = self.circle.center_y;
    circle.radius = self.circle.radius;
    circle
  }

  fn get_z(&self) -> f64 {
    BULLET_Z
  }

  fn is_active(&self) -> bool {
    self.active
  }

  fn is_updated(&self) -> bool {
    self.updated
  }
}
