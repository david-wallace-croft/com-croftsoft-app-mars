// =============================================================================
//! - Tank Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-30
//! - Updated: 2023-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::ai::tank_operator::TankOperator;
use crate::constant::TIME_DELTA;
use crate::world::World;
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct TankUpdater {
  world: Rc<dyn World>,
}

impl TankUpdater {
  pub fn new(
    // events: Rc<RefCell<dyn ClockUpdaterEvents>>,
    // inputs: Rc<RefCell<dyn ClockUpdaterInputs>>,
    // options: Rc<RefCell<dyn ClockUpdaterOptions>>,
    world: Rc<dyn World>,
  ) -> Self {
    Self {
      // events,
      // inputs,
      // options,
      world,
    }
  }
}

impl Updater for TankUpdater {
  fn update(&self) {
    // let inputs: Ref<dyn ClockUpdaterInputs> = self.inputs.borrow();
    // if inputs.get_reset_requested() {
    //   clock.time = 0;
    //   self.events.borrow_mut().set_updated();
    //   return;
    // }
    // if !inputs.get_time_to_update() || self.options.borrow().get_pause() {
    //   return;
    // }

    let tank_operators: Rc<RefCell<VecDeque<Box<dyn TankOperator>>>> =
      self.world.get_tank_operators();
    let length: usize = tank_operators.borrow().len();
    for _index in 0..length {
      // TOOD: Do we still need to pop?
      let tank_operator: Box<dyn TankOperator> =
        tank_operators.borrow_mut().pop_front().unwrap();
      let tank = tank_operator.get_tank();
      let mut tank = tank.borrow_mut();
      tank.update(TIME_DELTA);
      if tank.is_active() {
        tank_operators.borrow_mut().push_back(tank_operator);
      }
    }
  }
}
