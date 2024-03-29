// =============================================================================
//! - Options Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-13
//! - Updated: 2023-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::options::OptionsMutator;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub trait OptionsUpdaterInputs {
  fn get_node_display_change_requested(&self) -> Option<bool>;
  fn get_path_display_change_requested(&self) -> Option<bool>;
  fn get_pause_change_requested(&self) -> Option<bool>;
  // TODO: Are some of these unused?
  fn get_reset_requested(&self) -> bool;
  // fn get_time_display_change_requested(&self) -> Option<bool>;
  fn get_time_to_update(&self) -> bool;
  fn get_update_period_millis_changed(&self) -> Option<f64>;
  fn get_update_rate_display_change_requested(&self) -> Option<bool>;
  fn get_update_time_millis(&self) -> f64;
}

pub struct OptionsUpdater {
  inputs: Rc<RefCell<dyn OptionsUpdaterInputs>>,
  options: Rc<dyn OptionsMutator>,
}

impl OptionsUpdater {
  pub fn new(
    inputs: Rc<RefCell<dyn OptionsUpdaterInputs>>,
    options: Rc<dyn OptionsMutator>,
  ) -> Self {
    Self {
      inputs,
      options,
    }
  }
}

impl Updater for OptionsUpdater {
  // TODO: Does self need to be mutable?
  fn update(&self) {
    let inputs: Ref<dyn OptionsUpdaterInputs> = self.inputs.borrow();
    if let Some(frame_rate_display) =
      inputs.get_update_rate_display_change_requested()
    {
      self.options.set_update_rate_display(frame_rate_display);
    }
    if let Some(node_display) = inputs.get_node_display_change_requested() {
      self.options.set_node_display(node_display);
    }
    if let Some(path_display) = inputs.get_path_display_change_requested() {
      self.options.set_path_display(path_display);
    }
    if let Some(pause) = inputs.get_pause_change_requested() {
      self.options.set_pause(pause);
    }
    // if let Some(time_display) = inputs.get_time_display_change_requested() {
    //   self.options.borrow_mut().time_display = time_display;
    // }
  }
}
