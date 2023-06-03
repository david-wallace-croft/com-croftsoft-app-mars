// =============================================================================
//! - Component trait for CroftSoft Mars
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

use com_croftsoft_lib_role::{Initializer, Updater};

pub mod canvas;
pub mod root;
pub mod update_rate;

pub trait Component: Initializer + Updater {
  fn make_html(&self) -> String;
}
