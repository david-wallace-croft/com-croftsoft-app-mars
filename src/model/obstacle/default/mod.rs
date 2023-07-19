// =============================================================================
//! - Obstacle state for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-12
//! - Updated: 2023-07-19
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::{Obstacle, ObstacleAccessor};
use crate::constant::{
  OBSTACLE_JERK_MAGNITUDE_MAX, OBSTACLE_SPEED_MAX, OBSTACLE_Z,
};
use crate::model::{Damageable, Model, ModelAccessor};
use crate::world::World;
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use com_croftsoft_core::math::geom::point_2dd::Point2DD;
use com_croftsoft_core::math::geom::rectangle::Rectangle;
use com_croftsoft_lib_role::Preparer;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::rc::Weak;

pub struct DefaultObstacle {
  pub active: bool,
  pub circle: Circle,
  pub drift_bounds: Rectangle,
  // TODO: make this read-only
  pub id: usize,
  pub radius_min: f64,
  pub updated: bool,
  pub velocity_x: f64,
  pub velocity_y: f64,
  world: Weak<dyn World>,
}

impl DefaultObstacle {
  pub fn new(
    circle: Circle,
    drift_bounds: Rectangle,
    id: usize,
    radius_min: f64,
    world: Weak<dyn World>,
  ) -> Self {
    Self {
      active: true,
      circle,
      drift_bounds,
      id,
      radius_min,
      updated: false,
      velocity_x: 0.,
      velocity_y: 0.,
      world,
    }
  }
}

impl Damageable for DefaultObstacle {
  fn add_damage(
    &mut self,
    damage: f64,
  ) {
    if !self.active || damage <= 0. {
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

impl Model for DefaultObstacle {
  fn update(
    &mut self,
    time_delta: f64,
  ) {
    if !self.active {
      return;
    }
    let mut thread_rng: ThreadRng = rand::thread_rng();
    let velocity_x_delta: f64 = thread_rng.gen_range(-1.0..=1.0)
      * OBSTACLE_JERK_MAGNITUDE_MAX
      * time_delta;
    let velocity_y_delta: f64 = thread_rng.gen_range(-1.0..=1.0)
      * OBSTACLE_JERK_MAGNITUDE_MAX
      * time_delta;
    let mut velocity_x: f64 = self.velocity_x + velocity_x_delta;
    let mut velocity_y: f64 = self.velocity_y + velocity_y_delta;
    // TODO: clamp speed of vector instead of individual axis components
    velocity_x = velocity_x.clamp(-OBSTACLE_SPEED_MAX, OBSTACLE_SPEED_MAX);
    velocity_y = velocity_y.clamp(-OBSTACLE_SPEED_MAX, OBSTACLE_SPEED_MAX);
    let distance_x_delta: f64 = velocity_x * time_delta;
    let distance_y_delta: f64 = velocity_y * time_delta;
    let Circle {
      center_x: old_center_x,
      center_y: old_center_y,
      radius,
    } = self.circle;
    let mut new_center_x = old_center_x + distance_x_delta;
    let mut new_center_y = old_center_y + distance_y_delta;
    let max_center_x = self.drift_bounds.x_max - radius;
    let max_center_y = self.drift_bounds.y_max - radius;
    let min_center_x = self.drift_bounds.x_min + radius;
    let min_center_y = self.drift_bounds.y_min + radius;
    if new_center_x > max_center_x {
      new_center_x = max_center_x;
      velocity_x = -velocity_x;
    } else if new_center_x < min_center_x {
      new_center_x = min_center_x;
      velocity_x = -velocity_x;
    }
    if new_center_y > max_center_y {
      new_center_y = max_center_y;
      velocity_y = -velocity_y;
    } else if new_center_y < min_center_y {
      new_center_y = min_center_y;
      velocity_y = -velocity_y;
    }
    self.velocity_x = velocity_x;
    self.velocity_y = velocity_y;
    if new_center_x != old_center_x || new_center_y != old_center_y {
      if self.world.upgrade().unwrap().is_blocked_by_impassable(&self.circle) {
        self.circle.center_x = new_center_x;
        self.circle.center_y = new_center_y;
        // TODO: updated event
      } else {
        let new_circle = Circle {
          center_x: new_center_x,
          center_y: new_center_y,
          radius,
        };
        if !self.world.upgrade().unwrap().is_blocked_by_impassable(&new_circle)
        {
          self.circle.center_x = new_center_x;
          self.circle.center_y = new_center_y;
        } else {
          self.velocity_x = 0.;
          self.velocity_y = 0.;
          // TODO: updated event
        }
      }
    }
  }
}

impl ModelAccessor for DefaultObstacle {
  fn contains(
    &self,
    x: f64,
    y: f64,
  ) -> bool {
    self.circle.contains(x, y)
  }

  fn get_circle(&self) -> Circle {
    self.circle
  }

  fn get_id(&self) -> usize {
    self.id
  }

  fn get_z(&self) -> f64 {
    OBSTACLE_Z
  }

  fn intersects_circle(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool {
    self.circle.intersects_circle(circle)
  }

  fn is_active(&self) -> bool {
    self.active
  }

  fn is_updated(&self) -> bool {
    self.updated
  }
}

impl Obstacle for DefaultObstacle {}

impl ObstacleAccessor for DefaultObstacle {
  fn get_center(&self) -> Point2DD {
    self.circle.get_center_point_2dd()
  }
}

impl Preparer for DefaultObstacle {
  fn prepare(&mut self) {
    self.updated = false;
  }
}
