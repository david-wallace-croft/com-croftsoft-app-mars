// =============================================================================
//! - Default Ammo Dump for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-27
//! - Updated: 2023-05-20
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
use crate::models::world::World;
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use com_croftsoft_lib_role::Preparer;
use core::cell::RefCell;
use std::rc::Rc;

pub struct DefaultAmmoDump {
  ammo: f64,
  ammo_growth_rate: f64,
  ammo_max: f64,
  circle: Circle,
  exploding: bool,
  id: usize,
  updated: bool,
  world: Rc<RefCell<World>>,
  z: f64,
}

impl DefaultAmmoDump {
  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn new(
    ammo: f64,
    center_x: f64,
    center_y: f64,
    id: usize,
    world: Rc<RefCell<World>>,
  ) -> Self {
    let circle = Circle {
      center_x,
      center_y,
      radius: 0.,
    };
    let exploding = false;
    let updated = false;
    let mut ammo_dump = Self {
      ammo: 0.,
      ammo_growth_rate: AMMO_DUMP_AMMO_GROWTH_RATE,
      ammo_max: AMMO_DUMP_AMMO_MAX,
      circle,
      exploding,
      id,
      updated,
      world,
      z: AMMO_DUMP_Z,
    };
    ammo_dump.set_ammo(ammo);
    ammo_dump
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
    let mut explosion_circle = Circle::default();
    explosion_circle.set_center_from_circle(&self.circle);
    explosion_circle.radius = AMMO_DUMP_EXPLOSION_FACTOR * self.ammo;
    let explosion =
      self.world.borrow().factory.make_explosion(explosion_circle, self.ammo);
    self.world.borrow().explosions.borrow_mut().push_back(explosion);
    self.set_ammo(0.);
  }
}

impl Impassable for DefaultAmmoDump {}

impl Model for DefaultAmmoDump {
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
    time_delta: f64,
  ) {
    if self.exploding {
      return;
    }
    let mut new_ammo = self.ammo + time_delta * self.ammo_growth_rate;
    if new_ammo > self.ammo_max {
      new_ammo = self.ammo_max;
    } else {
      new_ammo = 0.;
    }
    self.set_ammo(new_ammo);
  }
}

impl ModelAccessor for DefaultAmmoDump {
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
    self.z
  }

  fn intersects_circle(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool {
    self.circle.intersects_circle(circle)
  }

  fn is_active(&self) -> bool {
    true
  }

  fn is_updated(&self) -> bool {
    self.updated
  }
}

impl Preparer for DefaultAmmoDump {
  fn prepare(&mut self) {
    self.exploding = false;
    self.updated = false;
  }
}
