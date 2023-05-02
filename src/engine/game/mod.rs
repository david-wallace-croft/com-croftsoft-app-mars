// =============================================================================
//! - Game for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-05-02
//! - Updated: 2023-05-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{
  OBSTACLE_COUNT, OBSTACLE_RADIUS_MAX, OBSTACLE_RADIUS_MIN,
  OBSTACLE_RANDOM_PLACEMENT_ATTEMPTS_MAX,
};
use crate::models::obstacle::state::ObstacleState;
use crate::models::tank::state::TankState;
use crate::state::root::Root;
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_core::math::geom::rectangle::Rectangle;
use core::cell::RefCell;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Game {
  // TODO
}

impl Game {
  fn make_level(&self) {
    todo!()
  }

  // TODO: make this private and call from make_level()
  pub fn make_obstacles(
    drift_bounds: Rectangle,
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
  ) -> VecDeque<ObstacleState> {
    let mut obstacles_vecdeque = VecDeque::<ObstacleState>::new();
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
      // TODO: make this using WorldBuilder
      let mut obstacle =
        ObstacleState::new(circle, drift_bounds, OBSTACLE_RADIUS_MIN);
      for _ in 0..OBSTACLE_RANDOM_PLACEMENT_ATTEMPTS_MAX {
        // TODO: Also check to see if blocked by something else
        if !Root::is_blocked_by_tank(&obstacle.circle, tanks.clone()) {
          break;
        }
        obstacle.circle.center_x = center_uniform.sample(&mut rng);
        obstacle.circle.center_y = center_uniform.sample(&mut rng);
      }
      obstacles_vecdeque.push_back(obstacle);
    }
    obstacles_vecdeque
  }

  pub fn update(&self) {
    // TODO: copy in the rest of the old code
    self.make_level();
  }
}
