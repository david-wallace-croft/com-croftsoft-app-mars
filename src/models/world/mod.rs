// =============================================================================
//! - World for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-29
//! - Updated: 2023-05-01
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::AMMO_DUMP_AMMO_MAX;
use crate::engine::traits::Color;
use crate::models::ammo_dump::default::DefaultAmmoDump;
use crate::models::obstacle::state::ObstacleState;
use crate::models::tank::state::TankState;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Default)]
pub struct World {
  pub ammo_dumps: Rc<RefCell<VecDeque<DefaultAmmoDump>>>,
  pub obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
  pub tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
}

#[derive(Default)]
pub struct WorldBuilder {
  pub world: Rc<RefCell<World>>,
}

impl WorldBuilder {
  pub fn build_ammo_dump(
    &self,
    center_x: f64,
    center_y: f64,
    id: usize,
  ) {
    let ammo_dump = DefaultAmmoDump::new(
      AMMO_DUMP_AMMO_MAX,
      center_x,
      center_y,
      id,
      self.world.clone(),
    );
    self.world.borrow().ammo_dumps.borrow_mut().push_back(ammo_dump);
  }

  pub fn build_tank(
    &self,
    center_x: f64,
    center_y: f64,
    color: Color,
    id: usize,
  ) -> Rc<RefCell<TankState>> {
    let tank_state = Rc::new(RefCell::new(TankState::new(
      center_x,
      center_y,
      color,
      id,
      self.world.clone(),
    )));
    // let tank_operator = Rc::new(RefCell::new(DefaultTankOperator::default()));
    // tank_state.borrow_mut().set_tank_operator(tank_operator.clone());
    // tank_operator.borrow_mut().set_tank_console(tank_state.clone());
    // TODO: model_array_keep.insert(seriTank) was in the old code here
    self.world.borrow().tanks.borrow_mut().push_back(tank_state.clone());
    tank_state
  }
}
