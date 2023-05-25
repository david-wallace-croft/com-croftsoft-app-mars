// =============================================================================
//! - Tank Preparer for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-13
//! - Updated: 2023-05-24
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::Tank;
use com_croftsoft_lib_role::Preparer;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct TankPreparer {
  tanks: Rc<RefCell<VecDeque<Rc<RefCell<dyn Tank>>>>>,
}

impl TankPreparer {
  pub fn new(tanks: Rc<RefCell<VecDeque<Rc<RefCell<dyn Tank>>>>>) -> Self {
    Self {
      tanks,
    }
  }
}

impl Preparer for TankPreparer {
  fn prepare(&mut self) {
    let length = self.tanks.borrow().len();
    for _index in 0..length {
      let tank = self.tanks.borrow_mut().pop_front().unwrap();
      tank.borrow_mut().prepare();
      self.tanks.borrow_mut().push_back(tank);
    }
  }
}
