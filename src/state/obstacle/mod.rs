// =============================================================================
//! - Obstacle state for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-12
//! - Updated: 2023-03-23
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_core::math::geom::rectangle::Rectangle;

pub struct ObstacleState {
  pub active: bool,
  pub circle: Circle,
  pub draft_bounds: Rectangle,
  pub radius_min: f64,
  pub updated: bool,
  pub velocity_x: f64,
  pub velocity_y: f64,
}

impl ObstacleState {
  pub fn new(circle: Circle) -> Self {
    Self {
      active: true,
      circle,
      draft_bounds: Rectangle::default(),
      radius_min: 0.,
      updated: false,
      velocity_x: 0.,
      velocity_y: 0.,
    }
  }
}
