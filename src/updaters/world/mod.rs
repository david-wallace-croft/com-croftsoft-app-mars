// =============================================================================
//! - World Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-30
//! - Updated: 2023-04-30
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::TIME_DELTA;
use crate::models::world::World;
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
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

pub struct WorldUpdater {
  world: Rc<RefCell<World>>,
}

impl WorldUpdater {
  pub fn new(
    // events: Rc<RefCell<dyn ClockUpdaterEvents>>,
    // inputs: Rc<RefCell<dyn ClockUpdaterInputs>>,
    // options: Rc<RefCell<dyn ClockUpdaterOptions>>,
    world: Rc<RefCell<World>>,
  ) -> Self {
    Self {
      // events,
      // inputs,
      // options,
      world,
    }
  }
}

impl Updater for WorldUpdater {
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
    // TODO: Use a child AmmoDumpsUpdater
    self
      .world
      .borrow()
      .ammo_dumps
      .borrow_mut()
      .iter_mut()
      .for_each(|ammo_dump| ammo_dump.update2(TIME_DELTA));
  }
}
