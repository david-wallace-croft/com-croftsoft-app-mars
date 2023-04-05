// =============================================================================
//! - Obstacle traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-27
//! - Updated: 2023-04-05
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::collision_detector::CollisionDetector;
use crate::engine::traits::{Damageable, Impassable, ModelAccessor};
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_core::math::geom::rectangle::Rectangle;

pub mod state;

pub trait Obstacle: Damageable + Impassable + ObstacleAccessor {
  fn set_active(
    &mut self,
    active: bool,
  );

  fn set_radius(
    &mut self,
    radius: f64,
  );

  fn update(
    &mut self,
    collision_detector: &CollisionDetector,
    drift_bounds: &Rectangle,
    time_delta: f64,
  );
}

pub trait ObstacleAccessor: ModelAccessor {
  fn get_circle(
    &self,
    circle: Circle,
  ) -> Circle;
}
