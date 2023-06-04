// =============================================================================
//! - Explosion Visitor for CroftSoft Mars
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
use crate::model::obstacle::Obstacle;
use crate::model::tank::Tank;
use crate::world::World;
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use std::rc::Rc;

pub struct ExplosionVisitor {
  world: Rc<dyn World>,
}

impl ExplosionVisitor {
  fn compute_explosion_damage(
    &self,
    circle: &dyn CircleAccessor,
  ) -> f64 {
    self
      .world
      .get_explosions()
      .borrow()
      .iter()
      .filter(|explosion| explosion.intersects_circle(circle))
      .fold(0., |damage, explosion| damage + explosion.get_damage())
  }

  pub fn new(world: Rc<dyn World>) -> Self {
    Self {
      world,
    }
  }
}

impl Visitor for ExplosionVisitor {
  fn visit_ammo_dump(
    &self,
    ammo_dump: &mut dyn AmmoDump,
  ) {
    let circle: Circle = ammo_dump.get_circle();
    let damage: f64 = self.compute_explosion_damage(&circle);
    ammo_dump.add_damage(damage);
  }

  // TODO: visit_bullet() could change bullet velocity if inside an explosion

  fn visit_obstacle(
    &self,
    obstacle: &mut dyn Obstacle,
  ) {
    let circle: Circle = obstacle.get_circle();
    let damage: f64 = self.compute_explosion_damage(&circle);
    obstacle.add_damage(damage);
  }

  fn visit_tank(
    &self,
    tank: &mut dyn Tank,
  ) {
    let circle: Circle = tank.get_circle();
    let damage: f64 = self.compute_explosion_damage(&circle);
    tank.add_damage(damage);
  }
}
