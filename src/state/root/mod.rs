// =============================================================================
//! - Root Model for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-03-11
//! - Updated: 2023-05-05
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::overlay::Overlay;
use crate::models::tank::state::TankState;
use crate::models::world::World;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// TODO: move others to world like obstacles was then replace Root with World
pub struct Root {
  pub overlay: Rc<RefCell<Overlay>>,
  pub tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
  pub world: Rc<RefCell<World>>,
}

impl Root {
  pub fn new(
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
    world: Rc<RefCell<World>>,
  ) -> Self {
    Self {
      overlay: Rc::new(RefCell::new(Overlay::default())),
      tanks,
      world,
    }
  }
}
