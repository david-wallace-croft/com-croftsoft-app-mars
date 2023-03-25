// =============================================================================
//! - Looper for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-03-24
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::components::root::RootComponent;
use crate::constants::CONFIGURATION;
use crate::messages::events::Events;
use crate::messages::inputs::Inputs;
use crate::state::configuration::Configuration;
use crate::state::obstacle::ObstacleState;
use crate::state::options::Options;
use crate::state::root::Root;
use crate::updaters::root::{RootUpdater, RootUpdaterConfiguration};
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_core::math::geom::rectangle::Rectangle;
use com_croftsoft_lib_animation::frame_rater::simple::SimpleFrameRater;
use com_croftsoft_lib_animation::frame_rater::FrameRater;
use com_croftsoft_lib_animation::web_sys::{spawn_local_loop, LoopUpdater};
use com_croftsoft_lib_role::{Initializer, Painter, Updater};
use core::cell::RefCell;
use std::collections::VecDeque;
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
    let Configuration {
      update_period_millis_initial,
    } = configuration;
    let root_updater_configuration = RootUpdaterConfiguration {
      update_period_millis_initial,
    };
    let frame_rater: Rc<RefCell<dyn FrameRater>> = Rc::new(RefCell::new(
      SimpleFrameRater::new(update_period_millis_initial),
    ));
    let events = Rc::new(RefCell::new(Events::default()));
    let inputs = Rc::new(RefCell::new(Inputs::default()));
    let options = Rc::new(RefCell::new(Options::default()));
    let mut obstacles_vecdeque = VecDeque::<ObstacleState>::new();
    for i in 1..=5 {
      let circle = Circle {
        center_x: (i * 100) as f64,
        center_y: (i * 100) as f64,
        radius: (i * i * 4) as f64,
      };
      let obstacle = ObstacleState::new(circle);
      obstacles_vecdeque.push_back(obstacle);
    }
    let obstacles = Rc::new(RefCell::new(obstacles_vecdeque));
    let root_state = Rc::new(RefCell::new(Root::new(obstacles)));
    let root_component = RootComponent::new(
      events.clone(),
      "root",
      inputs.clone(),
      options.clone(),
      root_state.clone(),
    );
    let drift_bounds = Rectangle {
      x_max: 600.,
      x_min: 0.,
      y_max: 600.,
      y_min: 0.,
    };
    let root_updater = RootUpdater::new(
      root_updater_configuration,
      drift_bounds,
      events.clone(),
      frame_rater,
      inputs.clone(),
      options,
      root_state,
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
