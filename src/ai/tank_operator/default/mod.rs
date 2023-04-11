// =============================================================================
//! - Default Tank Operater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-06
//! - Updated: 2023-04-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::TankOperator;
use crate::ai::state_space_node::StateSpaceNode;
use crate::ai::tank_cartographer::TankCartographer;
use crate::ai::tank_console::TankConsole;
use crate::constants::{A_STAR_DIRECTIONS, A_STAR_STEP_SIZE};
use com_croftsoft_core::ai::astar::structures::AStar;
use com_croftsoft_core::math::geom::point_2dd::Point2DD;
use core::cell::RefCell;
use std::rc::Rc;

pub struct DefaultTankOperator {
  a_star: AStar<TankCartographer, StateSpaceNode>,
  center: Point2DD,
  destination: Point2DD,
  // TODO: was PointXY
  enemy_center: Point2DD,
  start_state_space_node: StateSpaceNode,
  // tank_cartographer: TankCartographer,
  tank_console: Option<Rc<RefCell<dyn TankConsole>>>,
}

impl Default for DefaultTankOperator {
  fn default() -> Self {
    let tank_cartographer =
      TankCartographer::new(A_STAR_STEP_SIZE, A_STAR_DIRECTIONS);
    let a_star = AStar::new(tank_cartographer);
    let center = Point2DD::default();
    let destination = Point2DD::default();
    let enemy_center = Point2DD::default();
    let start_state_space_node = StateSpaceNode::default();
    let tank_console = None;
    Self {
      a_star,
      center,
      destination,
      enemy_center,
      start_state_space_node,
      // tank_cartographer,
      tank_console,
    }
  }
}

impl TankOperator for DefaultTankOperator {
  fn fire(&mut self) {
    // was empty in the old code
  }

  // TODO: was iterator
  fn get_path(&self) -> Vec<(f64, f64)> {
    todo!();
  }

  fn set_tank_console(
    &mut self,
    tank_console: Rc<RefCell<dyn TankConsole>>,
  ) {
    // TODO: old code would delegate to the existing tank cartographer
    let mut tank_cartographer =
      TankCartographer::new(A_STAR_STEP_SIZE, A_STAR_DIRECTIONS);
    tank_cartographer.set_tank_console(tank_console.clone());
    self.a_star = AStar::new(tank_cartographer);
    self.tank_console = Some(tank_console);
  }

  fn update(
    &mut self,
    time_delta: f64,
  ) {
    if self.tank_console.is_none() {
      return;
    }
    // TODO: left off here
    todo!();
    // let tank_console: Rc<RefCell<dyn TankConsole>> = self.tank_console.unwrap();
    // let tank_console = tank_console.borrow();
    // tank_console.get_center(&mut self.center);
  }
}
