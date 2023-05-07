// =============================================================================
//! - World Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-30
//! - Updated: 2023-05-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::TIME_DELTA;
use crate::models::obstacle::updater::ObstacleUpdater;
use crate::models::tank::updater::TankUpdater;
use crate::models::tank_operator::updater::TankOperatorUpdater;
use crate::models::world::World;
use com_croftsoft_core::math::geom::rectangle::Rectangle;
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::rc::Rc;

pub struct WorldUpdater {
  child_updaters: Vec<Box<dyn Updater>>,
  world: Rc<RefCell<World>>,
}

impl WorldUpdater {
  pub fn new(
    drift_bounds: Rectangle,
    world: Rc<RefCell<World>>,
  ) -> Self {
    let obstacles_updater =
      ObstacleUpdater::new(drift_bounds, world.borrow().obstacles.clone());
    let tank_operator_updater = TankOperatorUpdater::new(world.clone());
    let tank_updater = TankUpdater::new(world.borrow().tanks.clone());
    let child_updaters: Vec<Box<dyn Updater>> = vec![
      Box::new(tank_operator_updater),
      Box::new(tank_updater),
      Box::new(obstacles_updater),
    ];
    Self {
      child_updaters,
      world,
    }
  }
}

impl Updater for WorldUpdater {
  fn update(&mut self) {
    // TODO: Use a child AmmoDumpsUpdater
    self
      .world
      .borrow()
      .ammo_dumps
      .borrow_mut()
      .iter_mut()
      .for_each(|ammo_dump| ammo_dump.update2(TIME_DELTA));
    self.child_updaters.iter_mut().for_each(|updater| updater.update());
  }
}
