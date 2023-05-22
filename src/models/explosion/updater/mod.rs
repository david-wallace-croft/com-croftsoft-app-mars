// =============================================================================
//! - Explosion Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-15
//! - Updated: 2023-05-21
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::Explosion;
use crate::constants::TIME_DELTA;
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct ExplosionUpdater {
  explosions: Rc<RefCell<VecDeque<Box<dyn Explosion>>>>,
}

impl ExplosionUpdater {
  pub fn new(explosions: Rc<RefCell<VecDeque<Box<dyn Explosion>>>>) -> Self {
    Self {
      explosions,
    }
  }
}

impl Updater for ExplosionUpdater {
  fn update(&mut self) {
    let length: usize = self.explosions.borrow().len();
    for _index in 0..length {
      let mut explosion = self.explosions.borrow_mut().pop_front().unwrap();
      explosion.update(TIME_DELTA);
      if explosion.is_active() {
        self.explosions.borrow_mut().push_back(explosion);
      }
    }
  }
}
