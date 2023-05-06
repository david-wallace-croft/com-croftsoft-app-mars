// =============================================================================
//! - Obstacle state for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-12
//! - Updated: 2023-05-06
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::{Obstacle, ObstacleAccessor};
use crate::constants::{
  OBSTACLE_JERK_MAGNITUDE_MAX, OBSTACLE_SPEED_MAX, OBSTACLE_Z,
};
use crate::engine::traits::{Damageable, Impassable, Model, ModelAccessor};
use crate::models::world::World;
use crate::state::root::Root;
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_core::math::geom::rectangle::Rectangle;
use core::cell::RefCell;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::rc::Rc;

pub struct ObstacleState {
  pub active: bool,
  pub circle: Circle,
  pub drift_bounds: Rectangle,
  pub radius_min: f64,
  pub updated: bool,
  pub velocity_x: f64,
  pub velocity_y: f64,
  world: Rc<RefCell<World>>,
}

impl ObstacleState {
  pub fn new(
    circle: Circle,
    drift_bounds: Rectangle,
    radius_min: f64,
    world: Rc<RefCell<World>>,
  ) -> Self {
    Self {
      active: true,
      circle,
      drift_bounds,
      radius_min,
      updated: false,
      velocity_x: 0.,
      velocity_y: 0.,
      world,
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
    _root: Rc<RefCell<Root>>,
    _time_delta: f64,
  ) {
    // TODO: Move this to the updater
    todo!()
  }
}

impl ModelAccessor for ObstacleState {
  fn get_shape(
    &self,
    mut circle: Circle,
  ) -> Circle {
    circle.center_x = self.circle.center_x;
    circle.center_y = self.circle.center_y;
    circle.radius = self.circle.radius;
    circle
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

  fn update(
    &mut self,
    drift_bounds: &Rectangle,
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
    let max_center_x = drift_bounds.x_max - radius;
    let max_center_y = drift_bounds.y_max - radius;
    let min_center_x = drift_bounds.x_min + radius;
    let min_center_y = drift_bounds.y_min + radius;
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
      if self.world.borrow().is_blocked(&self.circle) {
        self.circle.center_x = new_center_x;
        self.circle.center_y = new_center_y;
        // TODO: updated event
      } else {
        let new_circle = Circle {
          center_x: new_center_x,
          center_y: new_center_y,
          radius,
        };
        if !self.world.borrow().is_blocked(&new_circle) {
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

impl ObstacleAccessor for ObstacleState {
  fn get_circle(
    &self,
    mut circle: Circle,
  ) -> Circle {
    circle.center_x = self.circle.center_x;
    circle.center_y = self.circle.center_y;
    circle.radius = self.circle.radius;
    circle
  }
}
