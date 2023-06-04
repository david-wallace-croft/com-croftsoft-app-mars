// =============================================================================
//! - Bullet traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-10
//! - Updated: 2023-06-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::model::{Model, ModelAccessor};

pub mod default;

pub trait Bullet: BulletAccessor + Model {
  fn mark_spent(&mut self);
}

pub trait BulletAccessor: ModelAccessor {
  fn get_damage(&self) -> f64;
}
