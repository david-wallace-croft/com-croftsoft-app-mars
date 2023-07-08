// =============================================================================
//! - Looper for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-07-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::component::root::RootComponent;
use crate::configuration::Configuration;
use crate::constant::CONFIGURATION;
use crate::root::default::DefaultRoot;
use crate::root::Root;
use crate::updater::root::RootUpdater;
use com_croftsoft_lib_animation::frame_rater::simple::SimpleFrameRater;
use com_croftsoft_lib_animation::frame_rater::FrameRater;
use com_croftsoft_lib_animation::web_sys::{spawn_local_loop, LoopUpdater};
use com_croftsoft_lib_role::{Initializer, Painter, Updater};
use core::cell::RefCell;
use std::rc::Rc;

// TODO: rename this
pub struct Looper {
  root: Rc<dyn Root>,
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
    let root = Rc::new(DefaultRoot::new(configuration));
    let root_component = RootComponent::new("root", root.clone());
    let root_updater =
      RootUpdater::new(frame_rater, root.clone(), root.clone());
    Self {
      root,
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
    self.root.get_inputs().borrow_mut().reset_requested = true;
  }
}

impl LoopUpdater for Looper {
  // TODO: rename this function
  fn update_loop(
    &mut self,
    update_time_millis: f64,
  ) {
    self.root.get_inputs().borrow_mut().current_time_millis =
      update_time_millis;
    self.root_component.update();
    self.root_updater.update();
    self.root_component.paint();
    self.root.get_events().borrow_mut().clear();
    self.root.get_inputs().borrow_mut().clear();
  }
}
