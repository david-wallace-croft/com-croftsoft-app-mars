// =============================================================================
//! - Default Bullet for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-10
//! - Updated: 2023-06-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::{Bullet, BulletAccessor};
use crate::constants::{
  BULLET_DAMAGE, BULLET_RADIUS, BULLET_RANGE, BULLET_VELOCITY, BULLET_Z,
};
use crate::models::{Model, ModelAccessor};
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use com_croftsoft_lib_role::Preparer;

pub struct DefaultBullet {
  active: bool,
  circle: Circle,
  distance: f64,
  heading: f64,
  id: usize,
  origin_x: f64,
  origin_y: f64,
  spent: bool,
  updated: bool,
}

impl DefaultBullet {
  pub fn new(
    heading: f64,
    id: usize,
    origin_x: f64,
    origin_y: f64,
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
      spent: false,
      updated: false,
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

  fn mark_spent(&mut self) {
    self.spent = true;
  }
}

impl BulletAccessor for DefaultBullet {
  fn get_damage(&self) -> f64 {
    if !self.active || self.spent {
      0.
    } else {
      BULLET_DAMAGE
    }
  }
}

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
    if self.spent {
      self.active = false;
      self.updated = true;
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

  fn get_id(&self) -> usize {
    self.id
  }

  fn get_z(&self) -> f64 {
    BULLET_Z
  }

  fn intersects_circle(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool {
    circle.contains(self.circle.center_x, self.circle.center_y)
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
