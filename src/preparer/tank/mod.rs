// =============================================================================
//! - Tank Preparer for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-13
//! - Updated: 2023-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::ai::tank_operator::TankOperator;
use com_croftsoft_lib_role::Preparer;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct TankPreparer {
  tank_operators: Rc<RefCell<VecDeque<Box<dyn TankOperator>>>>,
}

impl TankPreparer {
  pub fn new(
    tank_operators: Rc<RefCell<VecDeque<Box<dyn TankOperator>>>>
  ) -> Self {
    Self {
      tank_operators,
    }
  }
}

impl Preparer for TankPreparer {
  fn prepare(&self) {
    let length = self.tank_operators.borrow().len();
    for _index in 0..length {
      // TODO: probably no longer necessary to pop
      let tank_operator = self.tank_operators.borrow_mut().pop_front().unwrap();
      let tank = tank_operator.get_tank();
      tank.borrow_mut().prepare();
      self.tank_operators.borrow_mut().push_back(tank_operator);
    }
  }
}
