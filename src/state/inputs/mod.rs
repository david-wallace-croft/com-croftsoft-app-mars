// =============================================================================
//! - Inputs for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-03-11
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

// use crate::updaters::root::RootUpdaterInputs;

#[derive(Default)]
pub struct Inputs {
  pub current_time_millis: f64,
  pub pause_change_requested: Option<bool>,
  pub reset_requested: bool,
}

impl Inputs {
  pub fn clear(&mut self) {
    self.current_time_millis = 0.;
    self.pause_change_requested = None;
    self.reset_requested = false;
  }
}

// impl RootUpdaterInputs for Inputs {
//   fn get_current_time_millis(&self) -> f64 {
//     self.current_time_millis
//   }

//   fn get_pause_change_requested(&self) -> Option<bool> {
//     self.pause_change_requested
//   }

//   fn get_reset_requested(&self) -> bool {
//     self.reset_requested
//   }
// }
