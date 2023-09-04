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
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct TankOperatorUpdater {
  world: Rc<dyn World>,
}

impl TankOperatorUpdater {
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

impl Updater for TankOperatorUpdater {
  fn update(&self) {
    let tank_operators: Rc<RefCell<VecDeque<Box<dyn TankOperator>>>> =
      self.world.get_tank_operators();
    let length: usize = tank_operators.borrow().len();
    for _index in 0..length {
      // TODO: Probably do not need to pop
      let mut tank_operator: Box<dyn TankOperator> =
        tank_operators.borrow_mut().pop_front().unwrap();
      tank_operator.update(TIME_DELTA);
      tank_operators.borrow_mut().push_back(tank_operator);
    }
  }
}
