// =============================================================================
//! - World Preparer for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-13
//! - Updated: 2023-05-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

// use crate::models::ammo_dump::preparer::AmmoDumpPreparer;
// use crate::models::bullet::preparer::BulletPreparer;
// use crate::models::obstacle::preparer::ObstaclePreparer;
use crate::models::tank::preparer::TankPreparer;
// use crate::models::tank_operator::preparer::TankOperatorPreparer;
use crate::models::world::World;
use com_croftsoft_lib_role::Preparer;
use core::cell::RefCell;
use std::rc::Rc;

pub struct WorldPreparer {
  child_preparers: Vec<Box<dyn Preparer>>,
}

impl WorldPreparer {
  pub fn new(world: Rc<RefCell<World>>) -> Self {
    // let ammo_dump_preparer = AmmoDumpPreparer::new(world.clone());
    // let bullet_preparer = BulletPreparer::new(world.clone());
    // let obstacle_preparer = ObstaclePreparer::new(world.clone());
    // let tank_operator_preparer = TankOperatorPreparer::new(world.clone());
    let tank_preparer = TankPreparer::new(world);
    let child_preparers: Vec<Box<dyn Preparer>> = vec![
      // Box::new(ammo_dump_preparer),
      // Box::new(tank_operator_preparer),
      Box::new(tank_preparer),
      // Box::new(obstacle_preparer),
      // Box::new(bullet_preparer),
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
