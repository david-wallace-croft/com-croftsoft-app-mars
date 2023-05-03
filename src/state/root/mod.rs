// =============================================================================
//! - Root Model for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-03-11
//! - Updated: 2023-05-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::overlay::Overlay;
use crate::ai::tank_operator::TankOperator;
use crate::engine::traits::ModelAccessor;
use crate::models::tank::state::TankState;
use crate::models::world::World;
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// TODO: move others to world like obstacles was then replace Root with World
pub struct Root {
  pub overlay: Rc<RefCell<Overlay>>,
  pub tank_operators: Rc<RefCell<VecDeque<Rc<RefCell<dyn TankOperator>>>>>,
  pub tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
  pub world: Rc<RefCell<World>>,
}

impl Root {
  // TODO: Move this to World
  // TODO: argument was Model in old code; could be Shape
  pub fn is_blocked(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool {
    // TODO: Use CollisionDetector
    // TODO: Old code iterated over array of Impassable
    for obstacle in self.world.borrow().obstacles.borrow().iter() {
      if circle.intersects_circle(&obstacle.circle) {
        return true;
      }
    }
    Root::is_blocked_by_tank(circle, self.tanks.clone())
  }

  // TODO: Move this to Game or World
  pub fn is_blocked_by_tank(
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

  pub fn new(
    tank_operators: Rc<RefCell<VecDeque<Rc<RefCell<dyn TankOperator>>>>>,
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
    world: Rc<RefCell<World>>,
  ) -> Self {
    Self {
      overlay: Rc::new(RefCell::new(Overlay::default())),
      tank_operators,
      tanks,
      world,
    }
  }
}
