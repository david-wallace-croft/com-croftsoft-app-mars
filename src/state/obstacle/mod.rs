// =============================================================================
//! - Obstacle state for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2023-03-12
//! - Since: 2023-03-12
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_core::math::geom::structures::Circle;
use com_croftsoft_core::math::geom::structures::Rectangle;

#[derive(Default)]
pub struct Obstacle {
  pub active: bool,
  pub circle: Circle,
  pub draft_bounds: Rectangle,
  pub radius_min: f64,
  pub updated: bool,
  pub velocity_x: f64,
  pub velocity_y: f64,
}
