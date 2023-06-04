// =============================================================================
//! - Obstacle traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-27
//! - Updated: 2023-06-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::{Impassable, ModelAccessor};

pub mod default;

pub trait Obstacle: Impassable + ObstacleAccessor {
  fn set_active(
    &mut self,
    active: bool,
  );
}

pub trait ObstacleAccessor: ModelAccessor {}
