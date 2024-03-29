// =============================================================================
//! - Explosion Preparer for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-15
//! - Updated: 2023-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::model::explosion::Explosion;
use crate::world::World;
use com_croftsoft_lib_role::Preparer;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct ExplosionPreparer {
  explosions: Rc<RefCell<VecDeque<Box<dyn Explosion>>>>,
}

impl ExplosionPreparer {
  pub fn new(world: Rc<RefCell<dyn World>>) -> Self {
    let explosions = world.borrow().get_explosions();
    Self {
      explosions,
    }
  }
}

impl Preparer for ExplosionPreparer {
  fn prepare(&self) {
    let length: usize = self.explosions.borrow().len();
    for _index in 0..length {
      let mut explosion = self.explosions.borrow_mut().pop_front().unwrap();
      explosion.prepare();
      self.explosions.borrow_mut().push_back(explosion);
    }
  }
}
