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

use crate::constants::{
  OBSTACLE_JERK_MAGNITUDE_MAX, OBSTACLE_SPEED_MAX, TIME_DELTA,
};
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
    let mut thread_rng: ThreadRng = rand::thread_rng();
    let velocity_x_delta: f64 = thread_rng.gen_range(-1.0..=1.0)
      * OBSTACLE_JERK_MAGNITUDE_MAX
      * TIME_DELTA;
    let velocity_y_delta: f64 = thread_rng.gen_range(-1.0..=1.0)
      * OBSTACLE_JERK_MAGNITUDE_MAX
      * TIME_DELTA;
    let mut velocity_x: f64 = obstacle.velocity_x + velocity_x_delta;
    let mut velocity_y: f64 = obstacle.velocity_y + velocity_y_delta;
    // TODO: clamp speed of vector instead of individual axis components
    velocity_x = velocity_x.clamp(-OBSTACLE_SPEED_MAX, OBSTACLE_SPEED_MAX);
    velocity_y = velocity_y.clamp(-OBSTACLE_SPEED_MAX, OBSTACLE_SPEED_MAX);
    let distance_x_delta: f64 = velocity_x * TIME_DELTA;
    let distance_y_delta: f64 = velocity_y * TIME_DELTA;
    let Circle {
      center_x: old_center_x,
      center_y: old_center_y,
      radius,
    } = obstacle.circle;
    let mut new_center_x = old_center_x + distance_x_delta;
    let mut new_center_y = old_center_y + distance_y_delta;
    let max_center_x = self.drift_bounds.x_max - radius;
    let max_center_y = self.drift_bounds.y_max - radius;
    let min_center_x = self.drift_bounds.x_min + radius;
    let min_center_y = self.drift_bounds.y_min + radius;
    if new_center_x > max_center_x {
      new_center_x = max_center_x;
      velocity_x = -velocity_x;
    } else if new_center_x < min_center_x {
      new_center_x = min_center_x;
      velocity_x = -velocity_x;
    }
    if new_center_y > max_center_y {
      new_center_y = max_center_y;
      velocity_y = -velocity_y;
    } else if new_center_y < min_center_y {
      new_center_y = min_center_y;
      velocity_y = -velocity_y;
    }
    obstacle.velocity_x = velocity_x;
    obstacle.velocity_y = velocity_y;
    if new_center_x != old_center_x || new_center_y != old_center_y {
      obstacle.circle.center_x = new_center_x;
      obstacle.circle.center_y = new_center_y;
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
