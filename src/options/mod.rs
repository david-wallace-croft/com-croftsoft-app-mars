// =============================================================================
//! - Options Model for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-06-22
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub struct Options {
  pub path_display: bool,
  pub pause: bool,
  pub update_rate_display: bool,
}

impl Default for Options {
  fn default() -> Self {
    Self {
      path_display: true,
      pause: false,
      update_rate_display: false,
    }
  }
}
