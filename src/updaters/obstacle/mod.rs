// =============================================================================
//! - Obstacle Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-12
//! - Updated: 2023-03-16
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{OBSTACLE_DRIFT_RATE, OBSTACLE_SPEED_MAX};
use crate::state::obstacle::Obstacle;
use com_croftsoft_core::math::geom::structures::{Circle, Rectangle};
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::rc::Rc;
// TODO: Should I be using the js_sys random?
use rand::{rngs::ThreadRng, Rng};

// pub trait ClockUpdaterEvents {
//   fn set_updated(&mut self);
// }

// pub trait ClockUpdaterInputs {
//   fn get_reset_requested(&self) -> bool;
//   fn get_time_to_update(&self) -> bool;
// }

// pub trait ClockUpdaterOptions {
//   fn get_pause(&self) -> bool;
// }

pub struct ObstacleUpdater {
  drift_bounds: Rectangle,
  // events: Rc<RefCell<dyn ClockUpdaterEvents>>,
  // inputs: Rc<RefCell<dyn ClockUpdaterInputs>>,
  obstacles: Rc<RefCell<Vec<Obstacle>>>,
  // options: Rc<RefCell<dyn ClockUpdaterOptions>>,
}

impl ObstacleUpdater {
  pub fn new(
    drift_bounds: Rectangle,
    // events: Rc<RefCell<dyn ClockUpdaterEvents>>,
    // inputs: Rc<RefCell<dyn ClockUpdaterInputs>>,
    obstacles: Rc<RefCell<Vec<Obstacle>>>,
    // options: Rc<RefCell<dyn ClockUpdaterOptions>>,
  ) -> Self {
    Self {
      drift_bounds,
      // events,
      // inputs,
      obstacles,
      // options,
    }
  }

  fn update_obstacle(
    &self,
    obstacle: &mut Obstacle,
  ) {
    if !obstacle.active {
      return;
    }
    // TODO
    let time_delta = 1. / 60.;
    let mut thread_rng: ThreadRng = rand::thread_rng();
    let random_x: f64 = thread_rng.gen_range(-1.0..=1.0);
    let random_y: f64 = thread_rng.gen_range(-1.0..=1.0);
    obstacle.velocity_x += OBSTACLE_DRIFT_RATE * time_delta * random_x;
    obstacle.velocity_y += OBSTACLE_DRIFT_RATE * time_delta * random_y;
    if obstacle.velocity_x > OBSTACLE_SPEED_MAX {
      obstacle.velocity_x = OBSTACLE_SPEED_MAX;
    } else if obstacle.velocity_x < -OBSTACLE_SPEED_MAX {
      obstacle.velocity_x = -OBSTACLE_SPEED_MAX;
    }
    if obstacle.velocity_y > OBSTACLE_SPEED_MAX {
      obstacle.velocity_y = OBSTACLE_SPEED_MAX;
    } else if obstacle.velocity_y < -OBSTACLE_SPEED_MAX {
      obstacle.velocity_y = -OBSTACLE_SPEED_MAX;
    }
    let Circle {
      center_x,
      center_y,
      radius,
    } = obstacle.circle;
    let old_center_x = center_x;
    let old_center_y = center_y;
    let mut destination_x = old_center_x + obstacle.velocity_x * time_delta;
    let mut destination_y = old_center_y + obstacle.velocity_y * time_delta;
    let max_x = self.drift_bounds.x_max - radius;
    let max_y = self.drift_bounds.y_max - radius;
    let min_x = self.drift_bounds.x_min + radius;
    let min_y = self.drift_bounds.y_min + radius;
    if destination_x > max_x {
      destination_x = max_x;
      obstacle.velocity_x = -obstacle.velocity_x;
    } else if destination_x < min_x {
      destination_x = min_x;
      obstacle.velocity_x = -obstacle.velocity_x;
    }
    if destination_y > max_y {
      destination_y = max_y;
      obstacle.velocity_y = -obstacle.velocity_y;
    } else if destination_y < min_y {
      destination_y = min_y;
      obstacle.velocity_y = -obstacle.velocity_y;
    }
    if destination_x != old_center_x || destination_y != old_center_y {
      obstacle.circle.center_x = destination_x;
      obstacle.circle.center_y = destination_y;
      // TODO: verify not blocked
      // TODO: if block, revert center position
      // TODO: self.events.borrow_mut().set_updated();
    }
  }
}

impl Updater for ObstacleUpdater {
  fn update(&mut self) {
    // let inputs: Ref<dyn ClockUpdaterInputs> = self.inputs.borrow();
    // if inputs.get_reset_requested() {
    //   clock.time = 0;
    //   self.events.borrow_mut().set_updated();
    //   return;
    // }
    // if !inputs.get_time_to_update() || self.options.borrow().get_pause() {
    //   return;
    // }
    for obstacle in self.obstacles.borrow_mut().iter_mut() {
      self.update_obstacle(obstacle);
    }
  }
}
