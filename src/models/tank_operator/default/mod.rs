// =============================================================================
//! - Default Tank Operater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-06
//! - Updated: 2023-04-06
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use core::cell::RefCell;
use std::rc::Rc;
use crate::models::tank::TankConsole;
use super::TankOperator;

#[derive(Default)]
pub struct DefaultTankOperator {
}

impl TankOperator for DefaultTankOperator {
  fn fire(&mut self) {
    todo!();
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
