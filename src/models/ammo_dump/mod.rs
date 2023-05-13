// =============================================================================
//! -Ammo Dump traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-27
//! - Updated: 2023-05-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::{Damageable, Model, ModelAccessor};
use com_croftsoft_core::math::geom::circle::Circle;

pub mod default;
pub mod preparer;
pub mod updater;

pub trait AmmoDump: AmmoDumpAccessor + Damageable + Model {
  fn set_ammo(
    &mut self,
    ammo: f64,
  );
}

pub trait AmmoDumpAccessor: ModelAccessor {
  fn get_ammo(&self) -> f64;

  // TODO: Old code returned a Shape
  fn get_explosion_shape(&self) -> Circle;

  fn is_exploding(&self) -> bool;
}
