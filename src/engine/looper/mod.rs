// =============================================================================
//! - Looper for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-05-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::game::Game;
use super::traits::Color;
use super::world_builder::WorldBuilder;
use crate::ai::tank_console::default::DefaultTankConsole;
use crate::ai::tank_operator::default::DefaultTankOperator;
use crate::ai::tank_operator::TankOperator;
use crate::components::root::RootComponent;
use crate::constants::CONFIGURATION;
use crate::messages::events::Events;
use crate::messages::inputs::Inputs;
use crate::models::tank::state::TankState;
use crate::models::tank::TankMutator;
use crate::state::configuration::Configuration;
use crate::state::options::Options;
use crate::state::root::Root;
use crate::updaters::root::{RootUpdater, RootUpdaterConfiguration};
use com_croftsoft_core::math::geom::rectangle::Rectangle;
use com_croftsoft_lib_animation::frame_rater::simple::SimpleFrameRater;
use com_croftsoft_lib_animation::frame_rater::FrameRater;
use com_croftsoft_lib_animation::web_sys::{spawn_local_loop, LoopUpdater};
use com_croftsoft_lib_role::{Initializer, Painter, Updater};
use core::cell::RefCell;
use core::f64::consts::TAU;
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
    let drift_bounds = Rectangle {
      x_max: 600.,
      x_min: 0.,
      y_max: 600.,
      y_min: 0.,
    };
    let world_builder = WorldBuilder::default();
    for index in 0..5 {
      let offset = ((index + 1) * 100) as f64;
      world_builder.build_ammo_dump(offset, offset, index);
    }
    // TODO: left off here
    let mut tank_operators_vecdeque =
      VecDeque::<Rc<RefCell<dyn TankOperator>>>::new();
    let mut tanks_vecdeque = VecDeque::<Rc<RefCell<TankState>>>::new();
    for index in 0..6 {
      let center_x: f64 = if index >= 3 {
        (index * 200 - 500) as f64
      } else {
        (index * 200 + 100) as f64
      };
      let center_y: f64 = if index >= 3 {
        100.
      } else {
        500.
      };
      let color = if index >= 3 {
        Color::RED
      } else {
        Color::BLUE
      };
      let tank: Rc<RefCell<TankState>> =
        world_builder.build_tank(center_x, center_y, color, index);
      tank.borrow_mut().set_body_heading(((index) as f64) * TAU / 8.);
      tank.borrow_mut().set_turret_heading(((index) as f64) * TAU / 4.);
      tanks_vecdeque.push_back(tank.clone());
      let tank_operator = DefaultTankOperator::new(index);
      tank_operators_vecdeque.push_back(Rc::new(RefCell::new(tank_operator)));
    }
    let tank_operators = Rc::new(RefCell::new(tank_operators_vecdeque));
    let tanks = Rc::new(RefCell::new(tanks_vecdeque));
    let obstacles_vecdeque = Game::make_obstacles(drift_bounds, tanks.clone());
    let obstacles = Rc::new(RefCell::new(obstacles_vecdeque));
    let length = tanks.borrow().len();
    for index in 0..length {
      let tank = tanks.borrow()[index].clone();
      let obstacles = obstacles.clone();
      let tanks = tanks.clone();
      let tank_console = Rc::new(RefCell::new(DefaultTankConsole {
        obstacles,
        tank,
        tanks,
      }));
      tank_operators.borrow()[index]
        .borrow_mut()
        .set_tank_console(tank_console);
    }
    let root_state = Rc::new(RefCell::new(Root::new(
      obstacles,
      tank_operators,
      tanks,
      world_builder.world,
    )));
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
