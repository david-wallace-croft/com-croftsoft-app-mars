// =============================================================================
//! - Bullet Visitor for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-04
//! - Updated: 2023-06-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::Visitor;
use crate::model::ammo_dump::AmmoDump;
use crate::model::bullet::Bullet;
use crate::model::obstacle::Obstacle;
use crate::model::tank::Tank;
use crate::world::World;
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use std::rc::Rc;

pub struct BulletVisitor {
  world: Rc<dyn World>,
}

impl BulletVisitor {
  fn compute_bullet_damage(
    &self,
    circle: &dyn CircleAccessor,
  ) -> f64 {
    self
      .world
      .get_bullets()
      .borrow_mut()
      .iter_mut()
      .filter(|bullet| bullet.intersects_circle(circle))
      .fold(0., |damage: f64, bullet: &mut Box<dyn Bullet>| {
        let updated_damage: f64 = damage + bullet.get_damage();
        bullet.mark_spent();
        updated_damage
      })
  }

  pub fn new(world: Rc<dyn World>) -> Self {
    Self {
      world,
    }
  }
}

impl Visitor for BulletVisitor {
  fn visit_ammo_dump(
    &self,
    ammo_dump: &mut dyn AmmoDump,
  ) {
    let circle: Circle = ammo_dump.get_circle();
    let damage: f64 = self.compute_bullet_damage(&circle);
    ammo_dump.add_damage(damage);
  }

  fn visit_obstacle(
    &self,
    obstacle: &mut dyn Obstacle,
  ) {
    let circle: Circle = obstacle.get_circle();
    let damage: f64 = self.compute_bullet_damage(&circle);
    obstacle.add_damage(damage);
  }

  fn visit_tank(
    &self,
    tank: &mut dyn Tank,
  ) {
    let circle: Circle = tank.get_circle();
    let damage: f64 = self.compute_bullet_damage(&circle);
    tank.add_damage(damage);
  }
}
