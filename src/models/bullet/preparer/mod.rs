// =============================================================================
//! - Bullet Preparer for CroftSoft Mars
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

use super::Bullet;
use crate::models::world::World;
use com_croftsoft_lib_role::Preparer;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct BulletPreparer {
  bullets: Rc<RefCell<VecDeque<Box<dyn Bullet>>>>,
}

impl BulletPreparer {
  pub fn new(world: Rc<RefCell<World>>) -> Self {
    let bullets = world.borrow().bullets.clone();
    Self {
      bullets,
    }
  }
}

impl Preparer for BulletPreparer {
  fn prepare(&mut self) {
    self.bullets.borrow_mut().iter_mut().for_each(|bullet| bullet.prepare());
  }
}
