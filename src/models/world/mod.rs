// =============================================================================
//! - World for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-29
//! - Updated: 2023-04-29
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::AMMO_DUMP_AMMO_MAX;
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
  ) {
    let ammo_dump = DefaultAmmoDump::new(
      AMMO_DUMP_AMMO_MAX,
      center_x,
      center_y,
      self.world.clone(),
    );
    self.world.borrow().ammo_dumps.borrow_mut().push_back(ammo_dump);
  }
}
