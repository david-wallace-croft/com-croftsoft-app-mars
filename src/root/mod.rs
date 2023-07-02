// =============================================================================
//! - Root Model for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-07-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::game::Game;
use crate::overlay::Overlay;
use crate::world::World;
use core::cell::RefCell;
use std::rc::Rc;

pub struct Root {
  pub game: Rc<RefCell<Game>>,
  pub overlay: Rc<RefCell<Overlay>>,
  pub world: Rc<dyn World>,
}

impl Root {
  pub fn new(world: Rc<dyn World>) -> Self {
    let game = Rc::new(RefCell::new(Game {
      level: 1,
    }));
    Self {
      game,
      overlay: Rc::new(RefCell::new(Overlay::default())),
      world,
    }
  }
}
