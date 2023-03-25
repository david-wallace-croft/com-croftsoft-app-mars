// =============================================================================
//! - Obstacle state for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-12
//! - Updated: 2023-03-24
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::OBSTACLE_Z;
use crate::engine::traits::{
  Damageable, Impassable, Model, ModelAccessor, Obstacle, ObstacleAccessor,
  Shape,
};
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

impl Damageable for ObstacleState {
  fn add_damage(
    &mut self,
    damage: f64,
  ) {
    if !self.active || damage == 0. {
      return;
    }
    self.updated = true;
    let radius = self.circle.radius - damage;
    if radius < self.radius_min {
      self.active = false;
    } else {
      self.circle.radius = radius;
    }
  }
}

impl Impassable for ObstacleState {}

impl Model for ObstacleState {
  fn prepare(&mut self) {
    self.updated = false;
  }

  fn set_center(
    &mut self,
    x: f64,
    y: f64,
  ) {
    self.circle.center_x = x;
    self.circle.center_y = y;
  }

  fn update(
    &mut self,
    _time_delta: f64,
  ) {
    // TODO: Move this to the updater
    todo!()
  }
}

impl ModelAccessor for ObstacleState {
  fn get_shape(&self) -> Box<dyn Shape> {
    // Box::new(self.circle)
    todo!()
  }

  fn get_z(&self) -> f64 {
    OBSTACLE_Z
  }

  fn is_active(&self) -> bool {
    self.active
  }

  fn is_updated(&self) -> bool {
    self.updated
  }
}

impl Obstacle for ObstacleState {
  fn set_active(
    &mut self,
    active: bool,
  ) {
    self.active = active;
  }

  fn set_radius(
    &mut self,
    radius: f64,
  ) {
    self.circle.radius = radius;
  }
}

impl ObstacleAccessor for ObstacleState {}
