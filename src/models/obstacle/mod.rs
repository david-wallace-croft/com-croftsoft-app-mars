// =============================================================================
//! - Obstacle traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-27
//! - Updated: 2023-05-28
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::{Impassable, ModelAccessor};
use com_croftsoft_core::math::geom::circle::Circle;

pub mod default;
pub mod preparer;
pub mod updater;

pub trait Obstacle: Impassable + ObstacleAccessor {
  fn set_active(
    &mut self,
    active: bool,
  );

  fn set_radius(
    &mut self,
    radius: f64,
  );
}

pub trait ObstacleAccessor: ModelAccessor {
  fn get_circle(
    &self,
    circle: Circle,
  ) -> Circle;
}
