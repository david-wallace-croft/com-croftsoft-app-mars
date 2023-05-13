// =============================================================================
//! - Tank Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-30
//! - Updated: 2023-05-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::ai::tank_operator::TankOperator;
use crate::constants::TIME_DELTA;
use crate::models::world::World;
use com_croftsoft_lib_role::Updater;
use core::cell::{RefCell, RefMut};
use std::collections::VecDeque;
use std::rc::Rc;

pub struct TankOperatorUpdater {
  world: Rc<RefCell<World>>,
}

impl TankOperatorUpdater {
  pub fn new(
    // events: Rc<RefCell<dyn ClockUpdaterEvents>>,
    // inputs: Rc<RefCell<dyn ClockUpdaterInputs>>,
    // options: Rc<RefCell<dyn ClockUpdaterOptions>>,
    world: Rc<RefCell<World>>,
  ) -> Self {
    Self {
      // events,
      // inputs,
      // options,
      world,
    }
  }
}

impl Updater for TankOperatorUpdater {
  fn update(&mut self) {
    // let inputs: Ref<dyn ClockUpdaterInputs> = self.inputs.borrow();
    // if inputs.get_reset_requested() {
    //   clock.time = 0;
    //   self.events.borrow_mut().set_updated();
    //   return;
    // }
    // if !inputs.get_time_to_update() || self.options.borrow().get_pause() {
    //   return;
    // }
    let world = self.world.borrow();
    let mut tank_operators: RefMut<VecDeque<Rc<RefCell<dyn TankOperator>>>> =
      world.tank_operators.borrow_mut();
    let length = tank_operators.len();
    for _index in 0..length {
      let tank_operator = tank_operators.pop_front().unwrap();
      let tank = world.tanks.borrow_mut().pop_front().unwrap();
      tank_operator.borrow_mut().update(world.tanks.clone(), TIME_DELTA);
      tank_operators.push_back(tank_operator);
      world.tanks.borrow_mut().push_back(tank);
    }
  }
}
