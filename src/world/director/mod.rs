// =============================================================================
//! - World Builder Director for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023-2026 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-03
//! - Updated: 2026-08-30
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::World;
use super::builder::{WorldBuilder, WorldBuilderTankConfig};
use super::seed::WorldSeed;
use crate::constant::{
  AMMO_DUMP_AMMO_MAX, AMMO_DUMP_COUNT_MAXIMUM,
  AMMO_DUMP_RANDOM_PLACEMENT_ATTEMPTS_MAX, OBSTACLE_COUNT_MAXIMUM,
  OBSTACLE_RADIUS_MAX, OBSTACLE_RADIUS_MIN,
  OBSTACLE_RANDOM_PLACEMENT_ATTEMPTS_MAX, TANK_COUNT_MAXIMUM,
};
use crate::model::tank::Color;
use ::web_sys::js_sys::Math::random;
use com_croftsoft_core::math::geom::circle::Circle;
use core::f64::consts::FRAC_PI_2;
use std::rc::Rc;

pub struct WorldBuilderDirector {
  pub seed: WorldSeed,
  pub world_builder: WorldBuilder,
}

impl WorldBuilderDirector {
  pub fn direct(&self) {
    self.world_builder.world.upgrade().unwrap().clear();
    self.direct_tank_operators();
    self.direct_obstacles();
    self.direct_ammo_dumps();
  }

  fn direct_ammo_dumps(&self) {
    let world: &Rc<dyn World> = &self.world_builder.world.upgrade().unwrap();
    let ammo_dump_count = AMMO_DUMP_COUNT_MAXIMUM.min(self.seed.level);
    for index in 0..ammo_dump_count {
      let mut circle = Circle {
        center_x: 0.,
        center_y: 0.,
        radius: AMMO_DUMP_AMMO_MAX,
      };
      let x_min: f64 = self.seed.bounds.x_min + circle.radius + 1.;
      let x_max: f64 = self.seed.bounds.x_max - circle.radius - 1.;
      // let center_uniform: Uniform<f64> = Uniform::from(x_min..=x_max);
      for _ in 0..AMMO_DUMP_RANDOM_PLACEMENT_ATTEMPTS_MAX {
        circle.center_x = Self::uniform(x_min, x_max);
        circle.center_y = Self::uniform(x_min, x_max);
        if !world.is_blocked_by_impassable(&circle)
          && !world.is_blocked_by_ammo_dump(&circle)
        {
          break;
        }
      }
      self.world_builder.build_ammo_dump(
        circle.center_x,
        circle.center_y,
        index,
      );
    }
  }

  fn direct_obstacles(&self) {
    // let radius_uniform =
    //   Uniform::from(OBSTACLE_RADIUS_MIN..=OBSTACLE_RADIUS_MAX);
    let obstacle_count = OBSTACLE_COUNT_MAXIMUM.min(self.seed.level);
    for index in 0..obstacle_count {
      let mut circle = Circle {
        center_x: 0.,
        center_y: 0.,
        radius: Self::uniform(OBSTACLE_RADIUS_MIN, OBSTACLE_RADIUS_MAX),
      };
      let x_min = self.seed.bounds.x_min + circle.radius + 1.;
      let x_max = self.seed.bounds.x_max - circle.radius - 1.;
      // let center_uniform = Uniform::from(x_min..=x_max);
      for _ in 0..OBSTACLE_RANDOM_PLACEMENT_ATTEMPTS_MAX {
        circle.center_x = Self::uniform(x_min, x_max);
        circle.center_y = Self::uniform(x_min, x_max);
        if !self
          .world_builder
          .world
          .upgrade()
          .unwrap()
          .is_blocked_by_impassable(&circle)
        {
          break;
        }
      }
      self
        .world_builder
        .build_obstacle(circle, self.seed.bounds, index);
    }
  }

  fn direct_tank_operators(&self) {
    let heading_blue = -FRAC_PI_2;
    let heading_red = FRAC_PI_2;
    let tank_count = TANK_COUNT_MAXIMUM.min(self.seed.level);
    for index in 0..tank_count {
      let spacer_index = index.div_ceil(2);
      let delta_x: i64 = if index % 2 == 0 {
        spacer_index as i64 * 100
      } else {
        spacer_index as i64 * -100
      };
      self
        .world_builder
        .build_tank_operator(WorldBuilderTankConfig {
          body_heading: heading_blue,
          center_x: (300 + delta_x) as f64,
          center_y: 500.,
          color: Color::BLUE,
          id: index * 2,
          turret_heading: heading_blue,
        });
      self
        .world_builder
        .build_tank_operator(WorldBuilderTankConfig {
          body_heading: heading_red,
          center_x: (300 - delta_x) as f64,
          center_y: 100.,
          color: Color::RED,
          id: index * 2 + 1,
          turret_heading: heading_red,
        });
    }
  }

  // TODO: This should be x_min..=x_max but is currently x_min..x_max
  fn uniform(
    x_min: f64,
    x_max: f64,
  ) -> f64 {
    x_min + (x_max - x_min) * random()
  }
}
