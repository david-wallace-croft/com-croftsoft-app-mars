// =============================================================================
//! - Ammo Dump Preparer for CroftSoft Mars
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

use crate::model::ammo_dump::AmmoDump;
use com_croftsoft_lib_role::Preparer;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct AmmoDumpPreparer {
  ammo_dumps: Rc<RefCell<VecDeque<Box<dyn AmmoDump>>>>,
}

impl AmmoDumpPreparer {
  pub fn new(ammo_dumps: Rc<RefCell<VecDeque<Box<dyn AmmoDump>>>>) -> Self {
    Self {
      ammo_dumps,
    }
  }
}

impl Preparer for AmmoDumpPreparer {
  fn prepare(&self) {
    let length = self.ammo_dumps.borrow().len();
    for _ in 0..length {
      let mut ammo_dump = self.ammo_dumps.borrow_mut().pop_front().unwrap();
      ammo_dump.prepare();
      self.ammo_dumps.borrow_mut().push_back(ammo_dump);
    }
  }
}
