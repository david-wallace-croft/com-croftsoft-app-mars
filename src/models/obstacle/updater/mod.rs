// =============================================================================
//! - Obstacle Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-12
//! - Updated: 2023-05-20
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::TIME_DELTA;
use crate::engine::traits::Model;
use crate::models::obstacle::state::ObstacleState;
use crate::models::world::default::DefaultWorld;
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
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
  // events: Rc<RefCell<dyn ClockUpdaterEvents>>,
  // inputs: Rc<RefCell<dyn ClockUpdaterInputs>>,
  obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
  // options: Rc<RefCell<dyn ClockUpdaterOptions>>,
  // root: Rc<RefCell<Root>>,
}

impl ObstacleUpdater {
  pub fn new(
    // events: Rc<RefCell<dyn ClockUpdaterEvents>>,
    // inputs: Rc<RefCell<dyn ClockUpdaterInputs>>,
    // options: Rc<RefCell<dyn ClockUpdaterOptions>>,
    world: Rc<RefCell<DefaultWorld>>,
  ) -> Self {
    let obstacles = world.borrow().obstacles.clone();
    Self {
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
    let length: usize = self.obstacles.borrow().len();
    for _index in 0..length {
      let mut obstacle = self.obstacles.borrow_mut().pop_front().unwrap();
      obstacle.update(TIME_DELTA);
      self.obstacles.borrow_mut().push_back(obstacle);
    }
  }
}
