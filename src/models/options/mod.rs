// =============================================================================
//! - Options Model for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-05-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::painters::root::updater::RootUpdaterOptions;

pub mod updater;

#[derive(Default)]
pub struct Options {
  pub pause: bool,
  pub update_rate_display: bool,
}

impl RootUpdaterOptions for Options {
  fn get_pause(&self) -> bool {
    self.pause
  }

  fn get_update_rate_display(&self) -> bool {
    self.update_rate_display
  }
}
