// =============================================================================
//! - Default Root Model for CroftSoft Mars
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
use crate::game::default::DefaultGame;
use crate::game::Game;
use crate::options::default::DefaultOptions;
use crate::options::Options;
use crate::overlay::Overlay;
use crate::world::factory::default::DefaultWorldFactory;
use crate::world::factory::WorldFactory;
use crate::world::World;
use core::cell::RefCell;
use std::rc::Rc;

use super::Root;

pub struct DefaultRoot {
  configuration: Configuration,
  factory: Rc<dyn WorldFactory>,
  game: Rc<dyn Game>,
  options: Rc<dyn Options>,
  overlay: Rc<RefCell<Overlay>>,
  world: Rc<dyn World>,
}

impl DefaultRoot {
  pub fn new(configuration: Configuration) -> Self {
    let factory: Rc<dyn WorldFactory> = Rc::new(DefaultWorldFactory::default());
    let game = Rc::new(DefaultGame::default());
    let options = Rc::new(DefaultOptions::default());
    let overlay = Default::default();
    let world = factory.make_world();
    Self {
      configuration,
      factory,
      game,
      options,
      overlay,
      world,
    }
  }
}

impl Root for DefaultRoot {
  fn get_configuration(&self) -> Configuration {
    self.configuration.clone()
  }

  fn get_factory(&self) -> Rc<dyn WorldFactory> {
    self.factory.clone()
  }

  fn get_game(&self) -> Rc<dyn Game> {
    self.game.clone()
  }

  fn get_options(&self) -> Rc<dyn Options> {
    self.options.clone()
  }

  fn get_overlay(&self) -> Rc<RefCell<Overlay>> {
    self.overlay.clone()
  }

  fn get_world(&self) -> Rc<dyn World> {
    self.world.clone()
  }
}
