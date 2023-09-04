// =============================================================================
//! - Component trait for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-03
//! - Updated: 2023-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_lib_role::{InitializerMut, UpdaterMut};

pub mod canvas;
pub mod node;
pub mod path;
pub mod pause;
pub mod reset;
pub mod root;
pub mod update_rate;

pub trait Component: InitializerMut + UpdaterMut {
  fn make_html(&self) -> String;
}
