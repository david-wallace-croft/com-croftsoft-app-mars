// =============================================================================
//! - Bullet traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-10
//! - Updated: 2023-05-28
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::{Model, ModelAccessor};

pub mod default;
pub mod preparer;
pub mod updater;

pub trait Bullet: BulletAccessor + Model {
  fn fire(
    &mut self,
    origin_x: f64,
    origin_y: f64,
    heading: f64,
  );

  fn mark_spent(&mut self);
}

pub trait BulletAccessor: ModelAccessor {
  fn get_damage(&self) -> f64;
}
