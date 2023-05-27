// =============================================================================
//! - Default Explosion for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-15
//! - Updated: 2023-05-27
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::{Explosion, ExplosionAccessor};
use crate::engine::traits::{Model, ModelAccessor};
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use com_croftsoft_lib_role::Preparer;

#[derive(PartialEq)]
enum DefaultExplosionState {
  Exploding,
  Fading,
  Inactive,
}

pub struct DefaultExplosion {
  circle: Circle,
  damage: f64,
  id: usize,
  state: DefaultExplosionState,
  updated: bool,
}

impl DefaultExplosion {
  pub fn new(
    circle: Circle,
    damage: f64,
    id: usize,
  ) -> Self {
    Self {
      circle,
      damage,
      id,
      state: DefaultExplosionState::Exploding,
      updated: false,
    }
  }
}

impl Explosion for DefaultExplosion {}

impl ExplosionAccessor for DefaultExplosion {
  fn get_damage(&self) -> f64 {
    match self.state {
      DefaultExplosionState::Exploding => self.damage,
      _ => 0.,
    }
  }
}

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
    self.updated = true;
    if self.state == DefaultExplosionState::Exploding {
      self.state = DefaultExplosionState::Fading;
      return;
    }
    let radius_delta = self.circle.radius * time_delta;
    // TODO: Make this a constant
    self.circle.radius -= 10. * radius_delta;
    // TODO: Make this a constant
    if self.circle.radius < 1. {
      self.state = DefaultExplosionState::Inactive;
    }
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

  fn get_id(&self) -> usize {
    self.id
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
    self.state != DefaultExplosionState::Inactive
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
