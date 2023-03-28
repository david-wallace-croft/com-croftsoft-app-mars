// =============================================================================
//! - Obstacle traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-27
//! - Updated: 2023-03-27
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::{Damageable, Impassable, ModelAccessor};

pub mod painter;
pub mod state;
pub mod updater;

pub trait Obstacle: Damageable + Impassable + ObstacleAccessor {
  fn set_active(
    &mut self,
    active: bool,
  );
  fn set_radius(
    &mut self,
    radius: f64,
  );
}

pub trait ObstacleAccessor: ModelAccessor {}
