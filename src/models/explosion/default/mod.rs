// =============================================================================
//! - Default Explosion for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-15
//! - Updated: 2023-05-21
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::{Model, ModelAccessor};
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use com_croftsoft_lib_role::Preparer;

use super::Explosion;

pub struct DefaultExplosion {
  active: bool,
  circle: Circle,
  damage: f64,
  id: usize,
  updated: bool,
}

impl DefaultExplosion {
  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn new(
    circle: Circle,
    damage: f64,
    id: usize,
  ) -> Self {
    Self {
      active: true,
      circle,
      damage,
      id,
      updated: false,
    }
  }
}

impl Explosion for DefaultExplosion {}

impl Model for DefaultExplosion {
  fn set_center(
    &mut self,
    x: f64,
    y: f64,
  ) {
    // not needed
  }

  fn update(
    &mut self,
    time_delta: f64,
  ) {
    if !self.active {
      return;
    }
    let radius_delta = self.circle.radius * time_delta;
    // TODO: Make this a constant
    self.circle.radius -= 10. * radius_delta;
    self.updated = true;
    // TODO: Make this a constant
    self.active = self.circle.radius > 1.;
  }
}

impl ModelAccessor for DefaultExplosion {
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
    todo!()
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

impl Preparer for DefaultExplosion {
  fn prepare(&mut self) {
    self.updated = false;
  }
}
