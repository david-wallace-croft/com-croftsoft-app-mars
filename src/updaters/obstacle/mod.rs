// =============================================================================
//! - Obstacle Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-12
//! - Updated: 2023-03-12
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::state::obstacle::Obstacle;

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
  obstacle: Rc<RefCell<Obstacle>>,
  // options: Rc<RefCell<dyn ClockUpdaterOptions>>,
}

impl ObstacleUpdater {
  pub fn new(
    // events: Rc<RefCell<dyn ClockUpdaterEvents>>,
    // inputs: Rc<RefCell<dyn ClockUpdaterInputs>>,
    obstacle: Rc<RefCell<Obstacle>>,
    // options: Rc<RefCell<dyn ClockUpdaterOptions>>,
  ) -> Self {
    Self {
      // events,
      // inputs,
      obstacle,
      // options,
    }
  }
}

impl Updater for ObstacleUpdater {
  fn update(&mut self) {
    let mut obstacle: RefMut<Obstacle> = self.obstacle.borrow_mut();
    if !obstacle.active {
      return;
    }
    // velocity_x += DRIFT_RATE * time_delta * (2. * random.next() - 1.);

    // let mut clock: RefMut<Clock> = self.clock.borrow_mut();
    // let inputs: Ref<dyn ClockUpdaterInputs> = self.inputs.borrow();
    // if inputs.get_reset_requested() {
    //   clock.time = 0;
    //   self.events.borrow_mut().set_updated();
    //   return;
    // }
    // if !inputs.get_time_to_update() || self.options.borrow().get_pause() {
    //   return;
    // }
    // if clock.time >= GENES_MAX - 1 {
    //   clock.time = 0;
    // } else {
    //   clock.time += 1;
    // }
    // self.events.borrow_mut().set_updated();
  }
}
