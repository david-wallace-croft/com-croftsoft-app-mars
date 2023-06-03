// =============================================================================
//! -Ammo Dump traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-27
//! - Updated: 2023-06-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::models::{Model, ModelAccessor};

pub mod default;
pub mod preparer;

pub trait AmmoDump: AmmoDumpAccessor + Model {
  fn set_ammo(
    &mut self,
    ammo: f64,
  );
}

pub trait AmmoDumpAccessor: ModelAccessor {
  fn get_ammo(&self) -> f64;
}
