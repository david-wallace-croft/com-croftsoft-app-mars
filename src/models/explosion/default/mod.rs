// =============================================================================
//! - Default Explosion for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-15
//! - Updated: 2023-05-24
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::Explosion;
use crate::engine::traits::{Damageable, Model, ModelAccessor};
use crate::models::world::World;
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use com_croftsoft_lib_role::Preparer;
use std::rc::Rc;

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
  world: Rc<dyn World>,
}

impl DefaultExplosion {
  pub fn new(
    circle: Circle,
    damage: f64,
    id: usize,
    world: Rc<dyn World>,
  ) -> Self {
    Self {
      circle,
      damage,
      id,
      state: DefaultExplosionState::Exploding,
      updated: false,
      world,
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
    self.updated = true;
    if self.state == DefaultExplosionState::Exploding {
      self.state = DefaultExplosionState::Fading;
      // TODO: maybe world.add_damage(&self.circle, self.damage)
      self
        .world
        .get_ammo_dumps()
        .borrow_mut()
        .iter_mut()
        .filter(|ammo_dump| ammo_dump.intersects_circle(&self.circle))
        .for_each(|ammo_dump| ammo_dump.add_damage(self.damage));
      self
        .world
        .get_obstacles()
        .borrow_mut()
        .iter_mut()
        .filter(|obstacle| obstacle.intersects_circle(&self.circle))
        .for_each(|obstacle| obstacle.add_damage(self.damage));
      self
        .world
        .get_tanks()
        .borrow()
        .iter()
        .filter(|tank| tank.borrow_mut().intersects_circle(&self.circle))
        .for_each(|tank| tank.borrow_mut().add_damage(self.damage));
      return;
    }
    let radius_delta = self.circle.radius * time_delta;
    // TODO: Make this a constant
    self.circle.radius -= 10. * radius_delta;
    // TODO: Make this a constant
    if self.circle.radius > 1. {
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
