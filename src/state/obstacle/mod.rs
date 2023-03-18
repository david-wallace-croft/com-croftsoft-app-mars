// =============================================================================
//! - Obstacle state for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-12
//! - Updated: 2023-03-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::Entity;
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_core::math::geom::rectangle::Rectangle;
use uuid::Uuid;

pub struct Obstacle {
  pub active: bool,
  pub circle: Circle,
  pub draft_bounds: Rectangle,
  pub radius_min: f64,
  pub updated: bool,
  pub uuid: String,
  pub velocity_x: f64,
  pub velocity_y: f64,
}

impl Obstacle {
  pub fn new(circle: Circle) -> Self {
    Self {
      active: true,
      circle,
      draft_bounds: Rectangle::default(),
      radius_min: 0.,
      updated: false,
      uuid: Uuid::new_v4().to_string(),
      velocity_x: 0.,
      velocity_y: 0.,
    }
  }
}

impl Entity for Obstacle {
  fn has_uuid(
    &self,
    uuid: &str,
  ) -> bool {
    self.uuid.eq(uuid)
  }
}
