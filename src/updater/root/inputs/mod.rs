// =============================================================================
//! - Root Updater Inputs for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-03
//! - Updated: 2023-06-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::events::RootUpdaterEvents;
use crate::inputs::Inputs;
use crate::updater::options::OptionsUpdaterInputs;
use crate::updater::overlay::OverlayUpdaterInputs;
use com_croftsoft_lib_animation::frame_rater::updater::FrameRaterUpdaterInputs;
use com_croftsoft_lib_animation::metronome::updater::MetronomeUpdaterInputs;
use core::cell::RefCell;
use std::rc::Rc;

pub trait RootUpdaterInputs {
  fn get_current_time_millis(&self) -> f64;
  fn get_pause_change_requested(&self) -> Option<bool>;
  fn get_period_millis_change_requested(&self) -> Option<f64>;
  fn get_reset_requested(&self) -> bool;
  fn get_update_rate_display_change_requested(&self) -> Option<bool>;
}

impl RootUpdaterInputs for Inputs {
  fn get_current_time_millis(&self) -> f64 {
    self.current_time_millis
  }

  fn get_pause_change_requested(&self) -> Option<bool> {
    self.pause_change_requested
  }

  fn get_period_millis_change_requested(&self) -> Option<f64> {
    self.period_millis_change_requested
  }

  fn get_reset_requested(&self) -> bool {
    self.reset_requested
  }

  fn get_update_rate_display_change_requested(&self) -> Option<bool> {
    self.update_rate_display_change_requested
  }
}

pub struct RootUpdaterInputsAdapter {
  events: Rc<RefCell<dyn RootUpdaterEvents>>,
  inputs: Rc<RefCell<dyn RootUpdaterInputs>>,
}

impl RootUpdaterInputsAdapter {
  pub fn new(
    events: Rc<RefCell<dyn RootUpdaterEvents>>,
    inputs: Rc<RefCell<dyn RootUpdaterInputs>>,
  ) -> Self {
    Self {
      events,
      inputs,
    }
  }
}

impl FrameRaterUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_frame_rate_display_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_update_rate_display_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }

  fn get_update_period_millis_changed(&self) -> Option<f64> {
    self.events.borrow().get_update_period_millis_changed()
  }

  fn get_update_time_millis(&self) -> f64 {
    self.inputs.borrow().get_current_time_millis()
  }
}

impl MetronomeUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_current_time_millis(&self) -> f64 {
    self.inputs.borrow().get_current_time_millis()
  }

  fn get_period_millis_change_requested(&self) -> Option<f64> {
    self.inputs.borrow().get_period_millis_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }
}

impl OptionsUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_pause_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_pause_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  // fn get_time_display_change_requested(&self) -> Option<bool> {
  //   self.inputs.borrow().get_time_display_change_requested()
  // }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }

  fn get_update_period_millis_changed(&self) -> Option<f64> {
    self.events.borrow().get_update_period_millis_changed()
  }

  fn get_update_rate_display_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_update_rate_display_change_requested()
  }

  fn get_update_time_millis(&self) -> f64 {
    self.inputs.borrow().get_current_time_millis()
  }
}

impl OverlayUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_current_time_millis(&self) -> f64 {
    self.inputs.borrow().get_current_time_millis()
  }

  // fn get_pause_change_requested(&self) -> Option<bool> {
  //   self.inputs.borrow().get_pause_change_requested()
  // }

  // fn get_reset_requested(&self) -> bool {
  //   self.inputs.borrow().get_reset_requested()
  // }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }

  fn get_update_rate_display_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_update_rate_display_change_requested()
  }
}
