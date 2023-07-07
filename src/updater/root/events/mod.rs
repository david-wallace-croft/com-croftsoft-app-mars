// =============================================================================
//! - Root Updater Events for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-03
//! - Updated: 2023-07-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::events::Events;
use crate::updater::overlay::OverlayUpdaterEvents;
use com_croftsoft_lib_animation::metronome::updater::MetronomeUpdaterEvents;
use core::cell::RefCell;
use std::rc::Rc;

pub trait RootUpdaterEvents {
  fn get_updated(&self) -> bool;
  fn get_time_to_update(&self) -> bool;
  fn get_update_period_millis_changed(&self) -> Option<f64>;
  fn set_time_to_update(&mut self);
  fn set_update_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  );
  fn set_updated(&mut self);
}

impl RootUpdaterEvents for Events {
  fn get_time_to_update(&self) -> bool {
    self.time_to_update
  }

  fn get_update_period_millis_changed(&self) -> Option<f64> {
    self.update_period_millis_changed
  }

  fn get_updated(&self) -> bool {
    self.updated
  }

  fn set_time_to_update(&mut self) {
    self.time_to_update = true;
  }

  fn set_update_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  ) {
    self.update_period_millis_changed = Some(update_period_millis);
  }

  fn set_updated(&mut self) {
    self.updated = true;
  }
}

// TODO: get rid of adapters; maybe git rid of events
pub struct RootUpdaterEventsAdapter {
  events: Rc<RefCell<Events>>,
}

impl RootUpdaterEventsAdapter {
  pub fn new(events: Rc<RefCell<Events>>) -> Self {
    Self {
      events,
    }
  }
}

impl MetronomeUpdaterEvents for RootUpdaterEventsAdapter {
  fn set_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  ) {
    self
      .events
      .borrow_mut()
      .set_update_period_millis_changed(update_period_millis);
  }

  fn set_tick(&mut self) {
    self.events.borrow_mut().set_time_to_update();
  }
}

impl OverlayUpdaterEvents for RootUpdaterEventsAdapter {
  fn set_updated(&mut self) {
    self.events.borrow_mut().set_updated();
  }
}
