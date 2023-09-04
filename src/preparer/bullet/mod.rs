// =============================================================================
//! - Bullet Preparer for CroftSoft Mars
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

use crate::model::bullet::Bullet;
use com_croftsoft_lib_role::Preparer;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct BulletPreparer {
  bullets: Rc<RefCell<VecDeque<Box<dyn Bullet>>>>,
}

impl BulletPreparer {
  pub fn new(bullets: Rc<RefCell<VecDeque<Box<dyn Bullet>>>>) -> Self {
    Self {
      bullets,
    }
  }
}

impl Preparer for BulletPreparer {
  fn prepare(&self) {
    let length: usize = self.bullets.borrow().len();
    for _index in 0..length {
      let mut bullet = self.bullets.borrow_mut().pop_front().unwrap();
      bullet.prepare();
      self.bullets.borrow_mut().push_back(bullet);
    }
  }
}
