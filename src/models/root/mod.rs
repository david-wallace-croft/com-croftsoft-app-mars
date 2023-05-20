// =============================================================================
//! - Root Model for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-03-11
//! - Updated: 2023-05-20
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::models::overlay::Overlay;
use crate::models::world::default::DefaultWorld;
use core::cell::RefCell;
use std::rc::Rc;

pub struct Root {
  pub overlay: Rc<RefCell<Overlay>>,
  pub world: Rc<RefCell<DefaultWorld>>,
}

impl Root {
  pub fn new(world: Rc<RefCell<DefaultWorld>>) -> Self {
    Self {
      overlay: Rc::new(RefCell::new(Overlay::default())),
      world,
    }
  }
}
