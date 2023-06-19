// =============================================================================
//! - World Builder Director for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-03
//! - Updated: 2023-06-19
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::builder::WorldBuilder;
use super::factory::WorldFactory;
use super::seed::WorldSeed;
use super::World;
use crate::constant::{
  AMMO_DUMP_AMMO_MAX, AMMO_DUMP_RANDOM_PLACEMENT_ATTEMPTS_MAX,
  OBSTACLE_RADIUS_MAX, OBSTACLE_RADIUS_MIN,
  OBSTACLE_RANDOM_PLACEMENT_ATTEMPTS_MAX,
};
use crate::model::tank::Color;
use com_croftsoft_core::math::geom::circle::Circle;
use core::f64::consts::TAU;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::rngs::ThreadRng;
use std::rc::Rc;

pub struct WorldBuilderDirector {
  seed: WorldSeed,
  world_builder: WorldBuilder,
}

impl WorldBuilderDirector {
  pub fn direct(&self) -> Rc<dyn World> {
    self.direct_tank_operators();
    self.direct_obstacles();
    self.direct_ammo_dumps();
    self.world_builder.world.clone()
  }

  fn direct_ammo_dumps(&self) {
    let world: &Rc<dyn World> = &self.world_builder.world;
    let mut rng: ThreadRng = rand::thread_rng();
    for index in 0..self.seed.ammo_dump_count {
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
    for index in 0..self.seed.obstacle_count {
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
    for index in 0..6 {
      let center_x: f64 = if index >= 3 {
        (index * 200 - 500) as f64
      } else {
        (index * 200 + 100) as f64
      };
      let center_y: f64 = if index >= 3 {
        100.
      } else {
        500.
      };
      let color = if index >= 3 {
        Color::RED
      } else {
        Color::BLUE
      };
      let body_heading = ((index) as f64) * TAU / 8.;
      let turret_heading = ((index) as f64) * TAU / 4.;
      self.world_builder.build_tank_operator(
        body_heading,
        center_x,
        center_y,
        color,
        index,
        turret_heading,
      );
    }
  }

  pub fn new(
    factory: Rc<dyn WorldFactory>,
    seed: WorldSeed,
  ) -> Self {
    let world_builder = WorldBuilder::new(factory);
    Self {
      seed,
      world_builder,
    }
  }
}
