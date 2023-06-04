// =============================================================================
//! - Model traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-03
//! - Updated: 2023-06-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use com_croftsoft_lib_role::Preparer;

pub mod ammo_dump;
pub mod bullet;
pub mod explosion;
pub mod obstacle;
pub mod tank;

pub trait Damageable {
  fn add_damage(
    &mut self,
    damage: f64,
  );
}

pub trait Model: ModelAccessor + Preparer {
  fn update(
    &mut self,
    time_delta: f64,
  );
}

pub trait ModelAccessor {
  fn contains(
    &self,
    x: f64,
    y: f64,
  ) -> bool;

  fn get_circle(&self) -> Circle;

  fn get_id(&self) -> usize;

  fn get_z(&self) -> f64;

  fn intersects_circle(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool;

  fn is_active(&self) -> bool;

  fn is_updated(&self) -> bool;
}
