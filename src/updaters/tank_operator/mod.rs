// =============================================================================
//! - Tank Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-30
//! - Updated: 2023-04-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::ai::tank_operator::TankOperator;
use crate::constants::TIME_DELTA;
use crate::models::obstacle::state::ObstacleState;
use crate::models::tank::state::TankState;
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct TankOperatorUpdater {
  obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
  tank_operators: Rc<RefCell<VecDeque<Rc<RefCell<dyn TankOperator>>>>>,
  tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
}

impl TankOperatorUpdater {
  pub fn new(
    // events: Rc<RefCell<dyn ClockUpdaterEvents>>,
    // inputs: Rc<RefCell<dyn ClockUpdaterInputs>>,
    obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
    tank_operators: Rc<RefCell<VecDeque<Rc<RefCell<dyn TankOperator>>>>>,
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
    // options: Rc<RefCell<dyn ClockUpdaterOptions>>,
  ) -> Self {
    Self {
      // events,
      // inputs,
      obstacles,
      tank_operators,
      // options,
      tanks,
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
    let length = self.tank_operators.borrow().len();
    for _index in 0..length {
      let tank_operator = self.tank_operators.borrow_mut().pop_front().unwrap();
      let tank = self.tanks.borrow_mut().pop_front().unwrap();
      tank_operator.borrow_mut().update(
        self.obstacles.clone(),
        self.tanks.clone(),
        TIME_DELTA,
      );
      self.tank_operators.borrow_mut().push_back(tank_operator);
      self.tanks.borrow_mut().push_back(tank);
    }
  }
}
