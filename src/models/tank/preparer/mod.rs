// =============================================================================
//! - Tank Preparer for CroftSoft Mars
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

use crate::models::tank::state::TankState;
use crate::models::world::World;
use com_croftsoft_lib_role::Preparer;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct TankPreparer {
  tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
}

impl TankPreparer {
  pub fn new(world: Rc<RefCell<World>>) -> Self {
    let tanks = world.borrow().tanks.clone();
    Self {
      tanks,
    }
  }
}

impl Preparer for TankPreparer {
  fn prepare(&mut self) {
    self
      .tanks
      .borrow_mut()
      .iter_mut()
      .for_each(|tank| tank.borrow_mut().prepare());
  }
}
