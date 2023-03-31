// =============================================================================
//! - Root Model for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-03-11
//! - Updated: 2023-03-31
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::overlay::Overlay;
use crate::models::obstacle::state::ObstacleState;
use crate::models::tank::state::TankState;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Root {
  pub obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
  pub overlay: Rc<RefCell<Overlay>>,
  pub tanks: Rc<RefCell<VecDeque<TankState>>>,
}

impl Root {
  pub fn new(
    obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
    tanks: Rc<RefCell<VecDeque<TankState>>>,
  ) -> Self {
    Self {
      obstacles,
      overlay: Rc::new(RefCell::new(Overlay::default())),
      tanks,
    }
  }
}
