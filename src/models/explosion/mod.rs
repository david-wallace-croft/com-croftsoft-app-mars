// =============================================================================
//! - Explosion traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-15
//! - Updated: 2023-05-27
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::{Model, ModelAccessor};

pub mod default;
pub mod preparer;
pub mod updater;

pub trait Explosion: ExplosionAccessor + Model + ModelAccessor {}

pub trait ExplosionAccessor {
  fn get_damage(&self) -> f64;
}
