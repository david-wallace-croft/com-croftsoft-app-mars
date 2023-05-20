// =============================================================================
//! - Default World Factory for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-05-17
//! - Updated: 2023-05-20
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::WorldFactory;
use crate::models::bullet::default::DefaultBullet;
use crate::models::bullet::Bullet;
use crate::models::explosion::default::DefaultExplosion;
use crate::models::explosion::Explosion;
use crate::models::tank_operator::default::DefaultTankOperator;
use crate::models::tank_operator::TankOperator;
use crate::models::world::default::DefaultWorld;
use crate::models::world::World;
use com_croftsoft_core::math::geom::circle::Circle;
use core::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Default)]
pub struct DefaultWorldFactory {
  // TODO: maybe use an atomic instead of Cell for interior mutability
  id_next_bullet: Cell<usize>,
  id_next_explosion: Cell<usize>,
  id_next_tank_operator: Cell<usize>,
}

impl WorldFactory for DefaultWorldFactory {
  fn make_bullet(
    &self,
    heading: f64,
    origin_x: f64,
    origin_y: f64,
    world: Rc<dyn World>,
  ) -> Box<dyn Bullet> {
    let id = self.id_next_bullet.get();
    self.id_next_bullet.set(id + 1);
    let bullet = DefaultBullet::new(heading, id, origin_x, origin_y, world);
    Box::new(bullet)
  }

  fn make_explosion(
    &self,
    circle: Circle,
    damage: f64,
  ) -> Box<dyn Explosion> {
    let id = self.id_next_explosion.get();
    self.id_next_explosion.set(id + 1);
    let explosion = DefaultExplosion::new(circle, damage, id);
    Box::new(explosion)
  }

  fn make_tank_operator(&self) -> Rc<RefCell<dyn TankOperator>> {
    let id = self.id_next_tank_operator.get();
    self.id_next_tank_operator.set(id + 1);
    let tank_operator = DefaultTankOperator::new(id);
    Rc::new(RefCell::new(tank_operator))
  }

  fn make_world(&self) -> Rc<dyn World> {
    Rc::new(DefaultWorld::default())
  }
}
