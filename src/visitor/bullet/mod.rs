// =============================================================================
//! - Bullet Visitor for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-04
//! - Updated: 2023-07-28
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::Visitor;
use crate::constant::OBSTACLE_BULLET_DAMAGE_MULTIPLIER;
use crate::model::ammo_dump::AmmoDump;
use crate::model::obstacle::Obstacle;
use crate::model::tank::Tank;
use crate::world::World;
use com_croftsoft_core::math::geom::circle::Circle;
use std::rc::Weak;

pub struct BulletVisitor {
  world: Weak<dyn World>,
}

impl BulletVisitor {
  pub fn new(world: Weak<dyn World>) -> Self {
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
    if !ammo_dump.is_nominal() {
      return;
    }
    let circle: Circle = ammo_dump.get_circle();
    for bullet in self
      .world
      .upgrade()
      .unwrap()
      .get_bullets()
      .borrow_mut()
      .iter_mut()
    {
      let damage = bullet.get_damage();
      if damage <= 0. || !bullet.intersects_circle(&circle) {
        continue;
      }
      bullet.mark_spent();
      ammo_dump.add_damage(damage);
      return;
    }
  }

  fn visit_obstacle(
    &self,
    obstacle: &mut dyn Obstacle,
  ) {
    if !obstacle.is_active() {
      return;
    }
    for bullet in self
      .world
      .upgrade()
      .unwrap()
      .get_bullets()
      .borrow_mut()
      .iter_mut()
    {
      let damage = bullet.get_damage();
      if damage <= 0. {
        continue;
      }
      let circle: Circle = obstacle.get_circle();
      if !bullet.intersects_circle(&circle) {
        continue;
      }
      bullet.mark_spent();
      obstacle.add_damage(OBSTACLE_BULLET_DAMAGE_MULTIPLIER * damage);
      if !obstacle.is_active() {
        return;
      }
    }
  }

  fn visit_tank(
    &self,
    tank: &mut dyn Tank,
  ) {
    if !tank.is_active() {
      return;
    }
    let circle: Circle = tank.get_circle();
    for bullet in self
      .world
      .upgrade()
      .unwrap()
      .get_bullets()
      .borrow_mut()
      .iter_mut()
    {
      let damage = bullet.get_damage();
      if damage <= 0. || !bullet.intersects_circle(&circle) {
        continue;
      }
      bullet.mark_spent();
      tank.add_damage(damage);
      if !tank.is_active() {
        return;
      }
    }
  }
}
