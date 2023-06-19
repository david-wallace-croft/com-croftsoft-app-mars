// =============================================================================
//! - World Preparer for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-13
//! - Updated: 2023-06-19
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::preparer::ammo_dump::AmmoDumpPreparer;
use crate::preparer::bullet::BulletPreparer;
use crate::preparer::obstacle::ObstaclePreparer;
use crate::preparer::tank::TankPreparer;
use crate::world::World;
use com_croftsoft_lib_role::Preparer;
use std::rc::Rc;

pub struct WorldPreparer {
  child_preparers: Vec<Box<dyn Preparer>>,
}

impl WorldPreparer {
  pub fn new(world: Rc<dyn World>) -> Self {
    let ammo_dump_preparer = AmmoDumpPreparer::new(world.get_ammo_dumps());
    let bullet_preparer = BulletPreparer::new(world.get_bullets());
    let obstacle_preparer = ObstaclePreparer::new(world.get_obstacles());
    let tank_preparer = TankPreparer::new(world.get_tank_operators());
    let child_preparers: Vec<Box<dyn Preparer>> = vec![
      Box::new(ammo_dump_preparer),
      Box::new(tank_preparer),
      Box::new(obstacle_preparer),
      Box::new(bullet_preparer),
    ];
    Self {
      child_preparers,
    }
  }
}

impl Preparer for WorldPreparer {
  fn prepare(&mut self) {
    self.child_preparers.iter_mut().for_each(|preparer| preparer.prepare());
  }
}
