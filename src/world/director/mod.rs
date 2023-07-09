// =============================================================================
//! - World Builder Director for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-03
//! - Updated: 2023-07-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::builder::{WorldBuilder, WorldBuilderTankConfig};
use super::seed::WorldSeed;
use super::World;
use crate::constant::{
  AMMO_DUMP_AMMO_MAX, AMMO_DUMP_COUNT_MAXIMUM,
  AMMO_DUMP_RANDOM_PLACEMENT_ATTEMPTS_MAX, OBSTACLE_COUNT_MAXIMUM,
  OBSTACLE_RADIUS_MAX, OBSTACLE_RADIUS_MIN,
  OBSTACLE_RANDOM_PLACEMENT_ATTEMPTS_MAX, TANK_COUNT_MAXIMUM,
};
use crate::model::tank::Color;
use com_croftsoft_core::math::geom::circle::Circle;
use core::f64::consts::FRAC_PI_2;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::rngs::ThreadRng;
use std::rc::Rc;

pub struct WorldBuilderDirector {
  pub seed: WorldSeed,
  pub world_builder: WorldBuilder,
}

impl WorldBuilderDirector {
  pub fn direct(&self) {
    self.world_builder.world.clear();
    self.direct_tank_operators();
    self.direct_obstacles();
    self.direct_ammo_dumps();
  }

  fn direct_ammo_dumps(&self) {
    let world: &Rc<dyn World> = &self.world_builder.world;
    let mut rng: ThreadRng = rand::thread_rng();
    let ammo_dump_count = AMMO_DUMP_COUNT_MAXIMUM.min(self.seed.level);
    for index in 0..ammo_dump_count {
      let mut circle = Circle {
        center_x: 0.,
        center_y: 0.,
        radius: AMMO_DUMP_AMMO_MAX,
      };
      let x_min: f64 = self.seed.bounds.x_min + circle.radius + 1.;
      let x_max: f64 = self.seed.bounds.x_max - circle.radius - 1.;
      let center_uniform: Uniform<f64> = Uniform::from(x_min..=x_max);
      for _ in 0..AMMO_DUMP_RANDOM_PLACEMENT_ATTEMPTS_MAX {
        circle.center_x = center_uniform.sample(&mut rng);
        circle.center_y = center_uniform.sample(&mut rng);
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
    let mut rng = rand::thread_rng();
    let radius_uniform =
      Uniform::from(OBSTACLE_RADIUS_MIN..=OBSTACLE_RADIUS_MAX);
    let obstacle_count = OBSTACLE_COUNT_MAXIMUM.min(self.seed.level);
    for index in 0..obstacle_count {
      let mut circle = Circle {
        center_x: 0.,
        center_y: 0.,
        radius: radius_uniform.sample(&mut rng),
      };
      let x_min = self.seed.bounds.x_min + circle.radius + 1.;
      let x_max = self.seed.bounds.x_max - circle.radius - 1.;
      let center_uniform = Uniform::from(x_min..=x_max);
      for _ in 0..OBSTACLE_RANDOM_PLACEMENT_ATTEMPTS_MAX {
        circle.center_x = center_uniform.sample(&mut rng);
        circle.center_y = center_uniform.sample(&mut rng);
        if !self.world_builder.world.is_blocked_by_impassable(&circle) {
          break;
        }
      }
      self.world_builder.build_obstacle(circle, self.seed.bounds, index);
    }
  }

  fn direct_tank_operators(&self) {
    let heading_blue = -FRAC_PI_2;
    let heading_red = FRAC_PI_2;
    let tank_count = TANK_COUNT_MAXIMUM.min(self.seed.level);
    for index in 0..tank_count {
      let spacer_index = (index + 1) / 2;
      let delta_x: i64 = if index % 2 == 0 {
        spacer_index as i64 * 200
      } else {
        spacer_index as i64 * -200
      };
      self.world_builder.build_tank_operator(WorldBuilderTankConfig {
        body_heading: heading_blue,
        center_x: (300 + delta_x) as f64,
        center_y: 500.,
        color: Color::BLUE,
        id: index * 2,
        turret_heading: heading_blue,
      });
      self.world_builder.build_tank_operator(WorldBuilderTankConfig {
        body_heading: heading_red,
        center_x: (300 - delta_x) as f64,
        center_y: 100.,
        color: Color::RED,
        id: index * 2 + 1,
        turret_heading: heading_red,
      });
    }
  }
}
