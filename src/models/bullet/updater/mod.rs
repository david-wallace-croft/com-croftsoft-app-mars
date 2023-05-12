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
use std::rc::Rc;

pub struct BulletUpdater {
  bullets: Rc<RefCell<Vec<Box<dyn Bullet>>>>,
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
    self
      .bullets
      .borrow_mut()
      .iter_mut()
      .for_each(|bullet| bullet.update(TIME_DELTA));
  }
}
