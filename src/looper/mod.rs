// =============================================================================
//! - Looper for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-07-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::component::root::RootComponent;
use crate::configuration::Configuration;
use crate::constant::CONFIGURATION;
use crate::events::Events;
use crate::inputs::Inputs;
use crate::options::Options;
use crate::root::default::DefaultRoot;
use crate::updater::root::RootUpdater;
use com_croftsoft_lib_animation::frame_rater::simple::SimpleFrameRater;
use com_croftsoft_lib_animation::frame_rater::FrameRater;
use com_croftsoft_lib_animation::web_sys::{spawn_local_loop, LoopUpdater};
use com_croftsoft_lib_role::{Initializer, Painter, Updater};
use core::cell::RefCell;
use std::rc::Rc;

// TODO: rename this
pub struct Looper {
  events: Rc<RefCell<Events>>,
  inputs: Rc<RefCell<Inputs>>,
  root_component: RootComponent,
  root_updater: RootUpdater,
}

impl Looper {
  pub fn launch() {
    let mut looper = Looper::default();
    looper.initialize();
    spawn_local_loop(looper);
  }

  pub fn new(configuration: Configuration) -> Self {
    let frame_rater: Rc<RefCell<dyn FrameRater>> = Rc::new(RefCell::new(
      SimpleFrameRater::new(configuration.update_period_millis_initial),
    ));
    let events = Rc::new(RefCell::new(Events::default()));
    let inputs = Rc::new(RefCell::new(Inputs::default()));
    let options = Rc::new(RefCell::new(Options::default()));
    let root = Rc::new(DefaultRoot::default());
    let root_component = RootComponent::new(
      events.clone(),
      "root",
      inputs.clone(),
      options.clone(),
      root.clone(),
    );
    let root_updater = RootUpdater::new(
      configuration,
      events.clone(),
      frame_rater,
      inputs.clone(),
      options,
      root,
    );
    Self {
      events,
      inputs,
      root_component,
      root_updater,
    }
  }
}

impl Default for Looper {
  fn default() -> Self {
    Looper::new(CONFIGURATION)
  }
}

impl Initializer for Looper {
  fn initialize(&mut self) {
    self.root_component.initialize();
    self.inputs.borrow_mut().reset_requested = true;
  }
}

impl LoopUpdater for Looper {
  // TODO: rename this function
  fn update_loop(
    &mut self,
    update_time_millis: f64,
  ) {
    self.inputs.borrow_mut().current_time_millis = update_time_millis;
    self.root_component.update();
    self.root_updater.update();
    self.root_component.paint();
    self.events.borrow_mut().clear();
    self.inputs.borrow_mut().clear();
  }
}
