// =============================================================================
//! - Root Model for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-03-11
//! - Updated: 2023-04-24
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::overlay::Overlay;
use crate::ai::tank_operator::TankOperator;
use crate::constants::{
  OBSTACLE_COUNT, OBSTACLE_RADIUS_MAX, OBSTACLE_RADIUS_MIN,
  OBSTACLE_RANDOM_PLACEMENT_ATTEMPTS_MAX,
};
use crate::engine::traits::{Color, ModelAccessor};
use crate::models::obstacle::state::ObstacleState;
use crate::models::tank::state::TankState;
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use com_croftsoft_core::math::geom::rectangle::Rectangle;
use core::cell::RefCell;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Root {
  pub obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
  pub overlay: Rc<RefCell<Overlay>>,
  pub tank_operators: Rc<RefCell<VecDeque<Rc<RefCell<dyn TankOperator>>>>>,
  pub tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
}

impl Root {
  // TODO: argument was Model in old code; could be Shape
  pub fn is_blocked(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool {
    // TODO: Use CollisionDetector
    // TODO: Old code iterated over array of Impassable
    for obstacle in self.obstacles.borrow().iter() {
      if circle.intersects_circle(&obstacle.circle) {
        return true;
      }
    }
    Root::is_blocked_by_tank(circle, self.tanks.clone())
  }

  fn is_blocked_by_tank(
    circle: &dyn CircleAccessor,
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
  ) -> bool {
    let mut tank_circle = Circle::default();
    for tank in tanks.borrow().iter() {
      tank_circle = tank.borrow().get_shape(tank_circle);
      if circle.intersects_circle(&tank_circle) {
        return true;
      }
    }
    false
  }

  pub fn make_obstacles(
    drift_bounds: Rectangle,
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
  ) -> VecDeque<ObstacleState> {
    let mut obstacles_vecdeque = VecDeque::<ObstacleState>::new();
    let mut rng = rand::thread_rng();
    let center_uniform = Uniform::from(drift_bounds.x_min..=drift_bounds.x_max);
    let radius_uniform =
      Uniform::from(OBSTACLE_RADIUS_MIN..=OBSTACLE_RADIUS_MAX);
    for _ in 0..OBSTACLE_COUNT {
      let center_x = center_uniform.sample(&mut rng);
      let center_y = center_uniform.sample(&mut rng);
      let radius = radius_uniform.sample(&mut rng);
      let circle = Circle {
        center_x,
        center_y,
        radius,
      };
      let mut obstacle =
        ObstacleState::new(circle, drift_bounds, OBSTACLE_RADIUS_MIN);
      for _ in 0..OBSTACLE_RANDOM_PLACEMENT_ATTEMPTS_MAX {
        // TODO: Also check to see if blocked by something else
        if !Root::is_blocked_by_tank(&obstacle.circle, tanks.clone()) {
          break;
        }
        obstacle.circle.center_x = center_uniform.sample(&mut rng);
        obstacle.circle.center_y = center_uniform.sample(&mut rng);
      }
      obstacles_vecdeque.push_back(obstacle);
    }
    obstacles_vecdeque
  }

  pub fn make_tank(
    center_x: f64,
    center_y: f64,
    color: Color,
    id: usize,
  ) -> Rc<RefCell<TankState>> {
    let tank_state =
      Rc::new(RefCell::new(TankState::new(center_x, center_y, color, id)));
    // let tank_operator = Rc::new(RefCell::new(DefaultTankOperator::default()));
    // tank_state.borrow_mut().set_tank_operator(tank_operator.clone());
    // tank_operator.borrow_mut().set_tank_console(tank_state.clone());
    // TODO: model_array_keep.insert(seriTank) was in the old code here
    tank_state
  }

  pub fn new(
    obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
    tank_operators: Rc<RefCell<VecDeque<Rc<RefCell<dyn TankOperator>>>>>,
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
  ) -> Self {
    Self {
      obstacles,
      overlay: Rc::new(RefCell::new(Overlay::default())),
      tank_operators,
      tanks,
    }
  }
}
