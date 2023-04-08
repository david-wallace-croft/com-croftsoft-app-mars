// =============================================================================
//! - Default Tank Operater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-06
//! - Updated: 2023-04-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::TankOperator;
use crate::ai::tank_console::TankConsole;
use com_croftsoft_core::math::geom::point_2dd::Point2DD;
use core::cell::RefCell;
use std::rc::Rc;

pub struct DefaultTankOperator {
  // a_star: AStar,
  center: Point2DD,
  destination: Point2DD,
  // TODO: was PointXY
  enemy_center: Point2DD,
  // state_space_node: StateSpaceNode,
  // tank_cartographer: TankCartographer,
  tank_console: Rc<RefCell<dyn TankConsole>>,
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
    todo!();
  }

  fn update(
    &mut self,
    time_delta: f64,
  ) {
    todo!();
  }
}
