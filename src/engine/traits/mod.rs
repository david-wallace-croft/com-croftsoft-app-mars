// =============================================================================
//! - Traits for CroftSoft Mars
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

use com_croftsoft_lib_role::{Initializer, Updater};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
  // TODO: Support more than two colors
  BLUE,
  RED,
}

pub trait Component: Initializer + Updater {
  fn make_html(&self) -> String;
}

pub trait Shape {
  // TODO: java.awt.Shape
}
