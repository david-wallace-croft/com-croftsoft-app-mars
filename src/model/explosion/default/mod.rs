// =============================================================================
//! - Default Explosion for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-15
//! - Updated: 2023-06-20
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use self::state::State;
use super::{Explosion, ExplosionAccessor};
use crate::constant::{EXPLOSION_RADIUS_DECAY_RATE, EXPLOSION_RADIUS_MINIMUM};
use crate::model::{Model, ModelAccessor};
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use com_croftsoft_lib_role::Preparer;

pub mod state;

pub struct DefaultExplosion {
  circle: Circle,
  damage: f64,
  id: usize,
  state: State,
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
      state: State::default(),
      updated: false,
    }
  }
}

impl Explosion for DefaultExplosion {}

impl ExplosionAccessor for DefaultExplosion {
  fn get_damage(&self) -> f64 {
    match self.state {
      State::Exploding(_) => self.damage,
      _ => 0.,
    }
  }
}

impl Model for DefaultExplosion {
  fn update(
    &mut self,
    time_delta: f64,
  ) {
    match &mut self.state {
      State::Exploding(state_operator) => {
        self.state = state_operator.to_fading();
        self.updated = true;
      },
      State::Fading(state_operator) => {
        let radius_delta = self.circle.radius * time_delta;
        self.circle.radius -= EXPLOSION_RADIUS_DECAY_RATE * radius_delta;
        if self.circle.radius < EXPLOSION_RADIUS_MINIMUM {
          self.state = state_operator.to_inactive();
        }
        self.updated = true;
      },
      _ => (),
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
    !matches!(self.state, State::Inactive)
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
