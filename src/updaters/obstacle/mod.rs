// =============================================================================
//! - Obstacle Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-12
//! - Updated: 2023-04-05
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::TIME_DELTA;
use crate::engine::collision_detector::CollisionDetector;
use crate::models::obstacle::state::ObstacleState;
use crate::models::obstacle::Obstacle;
use com_croftsoft_core::math::geom::rectangle::Rectangle;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;

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
  collision_detector: Rc<RefCell<CollisionDetector>>,
  drift_bounds: Rectangle,
  // events: Rc<RefCell<dyn ClockUpdaterEvents>>,
  // inputs: Rc<RefCell<dyn ClockUpdaterInputs>>,
  obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
  // options: Rc<RefCell<dyn ClockUpdaterOptions>>,
}

impl ObstacleUpdater {
  pub fn new(
    collision_detector: Rc<RefCell<CollisionDetector>>,
    drift_bounds: Rectangle,
    // events: Rc<RefCell<dyn ClockUpdaterEvents>>,
    // inputs: Rc<RefCell<dyn ClockUpdaterInputs>>,
    obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
    // options: Rc<RefCell<dyn ClockUpdaterOptions>>,
  ) -> Self {
    Self {
      collision_detector,
      drift_bounds,
      // events,
      // inputs,
      obstacles,
      // options,
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
    let collision_detector: &Ref<CollisionDetector> =
      &self.collision_detector.borrow();
    let drift_bounds: &Rectangle = &self.drift_bounds;
    let length: usize = self.obstacles.borrow().len();
    for _index in 0..length {
      let mut obstacle = self.obstacles.borrow_mut().pop_front().unwrap();
      obstacle.update(collision_detector, drift_bounds, TIME_DELTA);
      self.obstacles.borrow_mut().push_back(obstacle);
    }
  }
}
