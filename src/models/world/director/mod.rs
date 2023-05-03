// =============================================================================
//! - World Director for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-05-03
//! - Updated: 2023-05-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::builder::WorldBuilder;
use crate::constants::{
  OBSTACLE_COUNT, OBSTACLE_RADIUS_MAX, OBSTACLE_RADIUS_MIN,
  OBSTACLE_RANDOM_PLACEMENT_ATTEMPTS_MAX,
};
use crate::models::obstacle::state::ObstacleState;
use crate::state::root::Root;
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_core::math::geom::rectangle::Rectangle;
use core::cell::RefCell;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct WorldDirector {
  // TODO
}

impl WorldDirector {
  fn make_level(&self) {
    todo!()
  }

  // TODO: make this private and call from make_level()
  // TODO: remove the return value
  pub fn make_obstacles(
    drift_bounds: Rectangle,
    world_builder: &WorldBuilder,
  ) -> Rc<RefCell<VecDeque<ObstacleState>>> {
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
        if !Root::is_blocked_by_tank(
          &obstacle.circle,
          world_builder.world.borrow().tanks.clone(),
        ) {
          break;
        }
        obstacle.circle.center_x = center_uniform.sample(&mut rng);
        obstacle.circle.center_y = center_uniform.sample(&mut rng);
      }
    }
    world_builder.world.borrow().obstacles.clone()
  }

  pub fn update(&self) {
    // TODO: copy in the rest of the old code
    self.make_level();
  }
}
