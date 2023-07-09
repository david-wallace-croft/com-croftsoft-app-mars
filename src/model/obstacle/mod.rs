// =============================================================================
//! - Obstacle traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-27
//! - Updated: 2023-07-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::{Damageable, Model, ModelAccessor};
use com_croftsoft_core::math::geom::point_2dd::Point2DD;

pub mod default;

pub trait Obstacle: Damageable + Model + ObstacleAccessor {}

pub trait ObstacleAccessor: ModelAccessor {
  fn get_center(&self) -> Point2DD;
}
