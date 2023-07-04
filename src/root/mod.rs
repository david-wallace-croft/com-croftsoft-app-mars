// =============================================================================
//! - Root trait for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-07-03
//! - Updated: 2023-07-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::configuration::Configuration;
use crate::game::Game;
use crate::overlay::Overlay;
use crate::world::factory::WorldFactory;
use crate::world::World;
use core::cell::RefCell;
use std::rc::Rc;

pub mod default;

pub trait Root {
  fn get_configuration(&self) -> Configuration;

  fn get_factory(&self) -> Rc<dyn WorldFactory>;

  fn get_game(&self) -> Rc<dyn Game>;

  fn get_overlay(&self) -> Rc<RefCell<Overlay>>;

  fn get_world(&self) -> Rc<dyn World>;
}
