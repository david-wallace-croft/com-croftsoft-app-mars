// =============================================================================
//! - Bullet Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-12
//! - Updated: 2023-05-12
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::Bullet;
use crate::constants::TIME_DELTA;
use crate::models::world::World;
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct BulletUpdater {
  bullets: Rc<RefCell<VecDeque<Box<dyn Bullet>>>>,
}

impl BulletUpdater {
  pub fn new(world: Rc<RefCell<World>>) -> Self {
    let bullets = world.borrow().bullets.clone();
    Self {
      bullets,
    }
  }
}

impl Updater for BulletUpdater {
  fn update(&mut self) {
    let length: usize = self.bullets.borrow().len();
    for _index in 0..length {
      let mut bullet = self.bullets.borrow_mut().pop_front().unwrap();
      bullet.update(TIME_DELTA);
      if bullet.is_active() {
        self.bullets.borrow_mut().push_back(bullet);
      }
    }
  }
}
