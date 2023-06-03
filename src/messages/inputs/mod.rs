// =============================================================================
//! - Inputs for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-06-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::updater::root::RootUpdaterInputs;

#[derive(Default)]
pub struct Inputs {
  pub current_time_millis: f64,
  pub pause_change_requested: Option<bool>,
  pub period_millis_change_requested: Option<f64>,
  pub reset_requested: bool,
  pub update_rate_display_change_requested: Option<bool>,
}

impl Inputs {
  pub fn clear(&mut self) {
    self.current_time_millis = 0.;
    self.pause_change_requested = None;
    self.period_millis_change_requested = None;
    self.reset_requested = false;
    self.update_rate_display_change_requested = None;
  }
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
