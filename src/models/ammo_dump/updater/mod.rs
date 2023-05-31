// =============================================================================
//! - Ammo Dump Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-07
//! - Updated: 2023-05-31
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::AmmoDump;
use crate::constants::TIME_DELTA;
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct AmmoDumpUpdater {
  ammo_dumps: Rc<RefCell<VecDeque<Box<dyn AmmoDump>>>>,
}

impl AmmoDumpUpdater {
  pub fn new(ammo_dumps: Rc<RefCell<VecDeque<Box<dyn AmmoDump>>>>) -> Self {
    Self {
      ammo_dumps,
    }
  }
}

impl Updater for AmmoDumpUpdater {
  fn update(&mut self) {
    let length = self.ammo_dumps.borrow().len();
    for _ in 0..length {
      let mut ammo_dump = self.ammo_dumps.borrow_mut().pop_front().unwrap();
      ammo_dump.update(TIME_DELTA);
      self.ammo_dumps.borrow_mut().push_back(ammo_dump);
    }
  }
}
