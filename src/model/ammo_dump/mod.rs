// =============================================================================
//! -Ammo Dump traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-27
//! - Updated: 2023-06-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::{Damageable, Model, ModelAccessor};

pub mod default;

pub trait AmmoDump: AmmoDumpAccessor + Damageable + Model {
  fn set_ammo(
    &mut self,
    ammo: f64,
  );
}

pub trait AmmoDumpAccessor: ModelAccessor {
  fn get_ammo(&self) -> f64;
}
