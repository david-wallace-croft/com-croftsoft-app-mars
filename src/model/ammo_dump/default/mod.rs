// =============================================================================
//! - Default Ammo Dump for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-27
//! - Updated: 2023-06-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use self::state::StateMachine;
use super::{AmmoDump, AmmoDumpAccessor};
use crate::constant::{
  AMMO_DUMP_AMMO_GROWTH_RATE, AMMO_DUMP_AMMO_MAX,
  AMMO_DUMP_COOLING_TIME_SECONDS, AMMO_DUMP_EXPLOSION_FACTOR, AMMO_DUMP_Z,
};
use crate::model::{Damageable, Model, ModelAccessor};
use crate::world::factory::WorldFactory;
use crate::world::World;
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use com_croftsoft_lib_role::Preparer;
use std::rc::Rc;

pub mod state;

pub struct DefaultAmmoDump {
  ammo: f64,
  ammo_growth_rate: f64,
  ammo_max: f64,
  circle: Circle,
  cooling_time_elapsed_seconds: f64,
  factory: Rc<dyn WorldFactory>,
  id: usize,
  state_machine: StateMachine,
  updated: bool,
  world: Rc<dyn World>,
  z: f64,
}

impl DefaultAmmoDump {
  pub fn new(
    ammo: f64,
    center_x: f64,
    center_y: f64,
    factory: Rc<dyn WorldFactory>,
    id: usize,
    world: Rc<dyn World>,
  ) -> Self {
    let circle = Circle {
      center_x,
      center_y,
      radius: 0.,
    };
    let mut ammo_dump = Self {
      ammo: 0.,
      ammo_growth_rate: AMMO_DUMP_AMMO_GROWTH_RATE,
      ammo_max: AMMO_DUMP_AMMO_MAX,
      circle,
      cooling_time_elapsed_seconds: 0.,
      factory,
      id,
      state_machine: StateMachine::default(),
      updated: false,
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
    self.circle.radius = if ammo > 0. {
      ammo
    } else {
      0.
    };
  }
}

impl AmmoDumpAccessor for DefaultAmmoDump {
  fn get_ammo(&self) -> f64 {
    self.ammo
  }

  fn is_nominal(&self) -> bool {
    matches!(self.state_machine, StateMachine::Nominal(_))
  }
}

impl Damageable for DefaultAmmoDump {
  fn add_damage(
    &mut self,
    damage: f64,
  ) {
    if damage <= 0. {
      return;
    }
    if let StateMachine::Nominal(state) = self.state_machine {
      self.state_machine = StateMachine::Exploding(state.explode());
      self.updated = true;
    }
  }
}

impl Model for DefaultAmmoDump {
  fn update(
    &mut self,
    time_delta: f64,
  ) {
    match self.state_machine {
      StateMachine::Cooling(state) => {
        self.cooling_time_elapsed_seconds += time_delta;
        if self.cooling_time_elapsed_seconds >= AMMO_DUMP_COOLING_TIME_SECONDS {
          self.state_machine = StateMachine::Nominal(state.reset());
        }
      },
      StateMachine::Exploding(state) => {
        self.state_machine = StateMachine::Cooling(state.cool());
        let mut explosion_circle = Circle::default();
        explosion_circle.set_center_from_circle(&self.circle);
        explosion_circle.radius = AMMO_DUMP_EXPLOSION_FACTOR * self.ammo;
        let explosion =
          self.factory.make_explosion(explosion_circle, self.ammo);
        self.world.add_explosion(explosion);
        self.set_ammo(0.);
        self.cooling_time_elapsed_seconds = 0.;
      },
      StateMachine::Nominal(_state) => {
        let mut new_ammo = self.ammo + time_delta * self.ammo_growth_rate;
        if new_ammo > self.ammo_max {
          new_ammo = self.ammo_max;
        } else if new_ammo < 0. {
          new_ammo = 0.;
        }
        self.set_ammo(new_ammo);
      },
    }
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

  fn get_id(&self) -> usize {
    self.id
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
    // self.exploding = false;
    self.updated = false;
  }
}
