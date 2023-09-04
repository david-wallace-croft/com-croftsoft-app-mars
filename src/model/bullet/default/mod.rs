// =============================================================================
//! - Default Bullet for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-10
//! - Updated: 2023-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::{Bullet, BulletAccessor};
use crate::constant::{
  BULLET_DAMAGE, BULLET_RADIUS, BULLET_RANGE, BULLET_VELOCITY, BULLET_Z,
};
use crate::model::{Model, ModelAccessor};
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use com_croftsoft_lib_role::PreparerMut;

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
    Self {
      active: true,
      circle: Circle {
        center_x: origin_x,
        center_y: origin_y,
        radius: BULLET_RADIUS,
      },
      distance: 0.,
      heading,
      id,
      origin_x,
      origin_y,
      spent: false,
      updated: true,
    }
  }
}

impl Bullet for DefaultBullet {
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

impl PreparerMut for DefaultBullet {
  fn prepare(&mut self) {
    self.updated = false;
  }
}
