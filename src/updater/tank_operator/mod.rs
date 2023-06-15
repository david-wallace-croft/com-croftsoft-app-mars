// =============================================================================
//! - Tank Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-30
//! - Updated: 2023-06-15
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constant::TIME_DELTA;
use crate::world::World;
use com_croftsoft_lib_role::Updater;
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
  fn update(&mut self) {
    let tanks = self.world.get_tanks();
    let length = tanks.borrow().len();
    for _index in 0..length {
      let tank = tanks.borrow_mut().pop_front().unwrap();
      let tank_operator = tank.borrow().get_tank_operator();
      if let Some(tank_operator) = tank_operator {
        tank_operator.borrow_mut().update(self.world.get_tanks(), TIME_DELTA);
      }
      tanks.borrow_mut().push_back(tank);
    }
  }
}
