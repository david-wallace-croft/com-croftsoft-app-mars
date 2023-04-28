// =============================================================================
//! - Default Ammo Dump for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-27
//! - Updated: 2023-04-28
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::{AmmoDump, AmmoDumpAccessor};
use crate::constants::{
  AMMO_DUMP_AMMO_GROWTH_RATE, AMMO_DUMP_AMMO_MAX, AMMO_DUMP_EXPLOSION_FACTOR,
  AMMO_DUMP_Z,
};
use crate::engine::traits::{Damageable, Impassable, Model, ModelAccessor};
use crate::state::root::Root;
use com_croftsoft_core::math::geom::circle::Circle;
use core::cell::RefCell;
use std::rc::Rc;

pub struct DefaultAmmoDump {
  ammo: f64,
  ammo_growth_rate: f64,
  ammo_max: f64,
  circle: Circle,
  exploding: bool,
  explosion_circle: Circle,
  explosion_factor: f64,
  updated: bool,
  z: f64,
}

impl DefaultAmmoDump {
  pub fn new(
    ammo: f64,
    center_x: f64,
    center_y: f64,
  ) -> Self {
    let circle = Circle {
      center_x,
      center_y,
      radius: 0.,
    };
    let exploding = false;
    let explosion_circle = Circle::default();
    let updated = false;
    Self {
      ammo,
      ammo_growth_rate: AMMO_DUMP_AMMO_GROWTH_RATE,
      ammo_max: AMMO_DUMP_AMMO_MAX,
      circle,
      exploding,
      explosion_circle,
      explosion_factor: AMMO_DUMP_EXPLOSION_FACTOR,
      updated,
      z: AMMO_DUMP_Z,
    }
  }
}

impl AmmoDump for DefaultAmmoDump {
  fn set_ammo(
    &mut self,
    ammo: f64,
  ) {
    if self.ammo == ammo {
      return;
    }
    self.ammo = ammo;
    self.updated = true;
    if ammo > 0. {
      self.circle.radius = ammo;
    } else {
      self.circle.radius = 0.;
    }
  }
}

impl AmmoDumpAccessor for DefaultAmmoDump {
  fn get_ammo(&self) -> f64 {
    self.ammo
  }

  fn get_explosion_shape(&self) -> Circle {
    self.explosion_circle
  }

  fn is_exploding(&self) -> bool {
    self.exploding
  }
}

impl Damageable for DefaultAmmoDump {
  fn add_damage(
    &mut self,
    _damage: f64,
  ) {
    if self.exploding {
      return;
    }
    self.updated = true;
    self.exploding = true;
    self.explosion_circle.set_center_from_circle(&self.circle);
    self.explosion_circle.radius = self.explosion_factor * self.ammo;
    // TODO: get damageables from root and add damage equal to ammo
    self.set_ammo(0.);
  }
}

impl Impassable for DefaultAmmoDump {}

impl Model for DefaultAmmoDump {
  fn prepare(&mut self) {
    self.exploding = false;
    self.updated = false;
  }

  fn set_center(
    &mut self,
    x: f64,
    y: f64,
  ) {
    self.circle.center_x = x;
    self.circle.center_y = y;
  }

  fn update(
    &mut self,
    _root: Rc<RefCell<Root>>,
    time_delta: f64,
  ) {
    // TODO: Move this to the updater
    if self.exploding {
      return;
    }
    let mut new_ammo = self.ammo + time_delta + self.ammo_growth_rate;
    if new_ammo > self.ammo_max {
      new_ammo = self.ammo_max;
    } else {
      new_ammo = 0.;
    }
    self.set_ammo(new_ammo);
  }
}

impl ModelAccessor for DefaultAmmoDump {
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
    self.z
  }

  fn is_active(&self) -> bool {
    true
  }

  fn is_updated(&self) -> bool {
    self.updated
  }
}
