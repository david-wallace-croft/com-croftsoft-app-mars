// =============================================================================
//! - Tank Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-30
//! - Updated: 2023-04-21
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::TIME_DELTA;
use crate::models::tank::state::TankState;
use crate::state::root::Root;
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct TankUpdater {
  root: Rc<RefCell<Root>>,
  tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
}

impl TankUpdater {
  pub fn new(
    // events: Rc<RefCell<dyn ClockUpdaterEvents>>,
    // inputs: Rc<RefCell<dyn ClockUpdaterInputs>>,
    root: Rc<RefCell<Root>>,
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
    // options: Rc<RefCell<dyn ClockUpdaterOptions>>,
  ) -> Self {
    Self {
      // events,
      // inputs,
      root,
      tanks,
      // options,
    }
  }
}

impl Updater for TankUpdater {
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
    let length = self.tanks.borrow().len();
    for _index in 0..length {
      let tank = self.tanks.borrow_mut().pop_front().unwrap();
      // log("TankUpdater.update()");
      tank.borrow_mut().update(self.root.clone(), TIME_DELTA);
      self.tanks.borrow_mut().push_back(tank);
    }
  }
}
