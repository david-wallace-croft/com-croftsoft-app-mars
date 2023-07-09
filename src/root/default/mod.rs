// =============================================================================
//! - Default Root Model for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-07-03
//! - Updated: 2023-07-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::configuration::Configuration;
use crate::events::Events;
use crate::game::default::DefaultGame;
use crate::game::Game;
use crate::inputs::Inputs;
use crate::options::default::DefaultOptions;
use crate::options::{Options, OptionsMutator};
use crate::overlay::Overlay;
use crate::world::factory::default::DefaultWorldFactory;
use crate::world::factory::WorldFactory;
use crate::world::World;
use core::cell::RefCell;
use std::rc::Rc;

use super::{Root, RootMutator};

pub struct DefaultRoot {
  configuration: Configuration,
  events: Rc<RefCell<Events>>,
  factory: Rc<DefaultWorldFactory>,
  game: Rc<DefaultGame>,
  inputs: Rc<RefCell<Inputs>>,
  options: Rc<DefaultOptions>,
  overlay: Rc<RefCell<Overlay>>,
  world: Rc<dyn World>,
}

impl DefaultRoot {
  pub fn new(configuration: Configuration) -> Self {
    let events: Rc<RefCell<Events>> = Default::default();
    let factory: Rc<DefaultWorldFactory> = Default::default();
    let game = Rc::new(DefaultGame::new(2));
    let inputs = Default::default();
    let options = Default::default();
    let overlay = Default::default();
    let world = factory.make_world();
    Self {
      configuration,
      events,
      factory,
      game,
      inputs,
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

  fn get_events(&self) -> Rc<RefCell<Events>> {
    self.events.clone()
  }

  fn get_factory(&self) -> Rc<dyn WorldFactory> {
    self.factory.clone()
  }

  fn get_game(&self) -> Rc<dyn Game> {
    self.game.clone()
  }

  fn get_inputs(&self) -> Rc<RefCell<Inputs>> {
    self.inputs.clone()
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

impl RootMutator for DefaultRoot {
  fn get_options_mutator(&self) -> Rc<dyn OptionsMutator> {
    self.options.clone()
  }
}
