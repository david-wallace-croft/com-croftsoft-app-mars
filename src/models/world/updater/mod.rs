// =============================================================================
//! - World Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-30
//! - Updated: 2023-05-20
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::World;
use crate::models::ammo_dump::updater::AmmoDumpUpdater;
use crate::models::bullet::updater::BulletUpdater;
use crate::models::obstacle::updater::ObstacleUpdater;
use crate::models::tank::updater::TankUpdater;
use crate::models::tank_operator::updater::TankOperatorUpdater;
use com_croftsoft_lib_role::Updater;
use std::rc::Rc;

pub struct WorldUpdater {
  child_updaters: Vec<Box<dyn Updater>>,
}

impl WorldUpdater {
  pub fn new(world: Rc<dyn World>) -> Self {
    let ammo_dump_updater = AmmoDumpUpdater::new(world.get_ammo_dumps());
    let bullet_updater = BulletUpdater::new(world.get_bullets());
    let obstacle_updater = ObstacleUpdater::new(world.get_obstacles());
    let tank_operator_updater = TankOperatorUpdater::new(world.clone());
    let tank_updater = TankUpdater::new(world.get_tanks());
    let child_updaters: Vec<Box<dyn Updater>> = vec![
      Box::new(ammo_dump_updater),
      Box::new(tank_operator_updater),
      Box::new(tank_updater),
      Box::new(obstacle_updater),
      Box::new(bullet_updater),
    ];
    Self {
      child_updaters,
    }
  }
}

impl Updater for WorldUpdater {
  fn update(&mut self) {
    self.child_updaters.iter_mut().for_each(|updater| updater.update());
  }
}
