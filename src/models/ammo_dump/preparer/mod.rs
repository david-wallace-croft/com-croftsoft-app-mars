// =============================================================================
//! - Ammo Dump Preparer for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-13
//! - Updated: 2023-05-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::default::DefaultAmmoDump;
use crate::models::world::World;
use com_croftsoft_lib_role::Preparer;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct AmmoDumpPreparer {
  ammo_dumps: Rc<RefCell<VecDeque<DefaultAmmoDump>>>,
}

impl AmmoDumpPreparer {
  pub fn new(world: Rc<RefCell<World>>) -> Self {
    let ammo_dumps = world.borrow().ammo_dumps.clone();
    Self {
      ammo_dumps,
    }
  }
}

impl Preparer for AmmoDumpPreparer {
  fn prepare(&mut self) {
    self
      .ammo_dumps
      .borrow_mut()
      .iter_mut()
      .for_each(|ammo_dump| ammo_dump.prepare());
  }
}
