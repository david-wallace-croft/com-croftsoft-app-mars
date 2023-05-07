// =============================================================================
//! - World Director for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-05-03
//! - Updated: 2023-05-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::builder::WorldBuilder;
use crate::ai::tank_console::default::DefaultTankConsole;
use crate::constants::{
  OBSTACLE_COUNT, OBSTACLE_RADIUS_MAX, OBSTACLE_RADIUS_MIN,
  OBSTACLE_RANDOM_PLACEMENT_ATTEMPTS_MAX,
};
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_core::math::geom::rectangle::Rectangle;
use core::cell::RefCell;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use std::rc::Rc;

pub struct WorldDirector {
  // TODO
}

impl WorldDirector {
  fn make_level(&self) {
    todo!()
  }

  // TODO: make this private and call from make_level()
  pub fn make_obstacles(
    drift_bounds: Rectangle,
    world_builder: &WorldBuilder,
  ) {
    let mut rng = rand::thread_rng();
    let center_uniform = Uniform::from(drift_bounds.x_min..=drift_bounds.x_max);
    let radius_uniform =
      Uniform::from(OBSTACLE_RADIUS_MIN..=OBSTACLE_RADIUS_MAX);
    for _ in 0..OBSTACLE_COUNT {
      let center_x = center_uniform.sample(&mut rng);
      let center_y = center_uniform.sample(&mut rng);
      let radius = radius_uniform.sample(&mut rng);
      let circle = Circle {
        center_x,
        center_y,
        radius,
      };
      world_builder.build_obstacle(circle, drift_bounds);
    }
    for mut obstacle in
      world_builder.world.borrow().obstacles.borrow_mut().iter_mut()
    {
      for _ in 0..OBSTACLE_RANDOM_PLACEMENT_ATTEMPTS_MAX {
        // TODO: Also check to see if blocked by something else
        if world_builder.world.borrow().is_blocked_by_tank(&obstacle.circle) {
          break;
        }
        obstacle.circle.center_x = center_uniform.sample(&mut rng);
        obstacle.circle.center_y = center_uniform.sample(&mut rng);
      }
    }
  }

  pub fn make_tank_consoles(world_builder: &WorldBuilder) {
    let world = world_builder.world.borrow();
    let tanks = world.tanks.borrow();
    let length = tanks.len();
    for index in 0..length {
      let tank = tanks[index].clone();
      let tank_console = Rc::new(RefCell::new(DefaultTankConsole {
        tank,
        world: world_builder.world.clone(),
      }));
      world_builder.world.borrow().tank_operators.borrow()[index]
        .borrow_mut()
        .set_tank_console(tank_console);
    }
  }

  pub fn update(&self) {
    // TODO: copy in the rest of the old code
    self.make_level();
  }
}
