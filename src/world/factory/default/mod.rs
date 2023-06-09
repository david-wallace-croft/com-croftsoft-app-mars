// =============================================================================
//! - Default World Factory for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-17
//! - Updated: 2023-06-15
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::WorldFactory;
use crate::model::bullet::default::DefaultBullet;
use crate::model::bullet::Bullet;
use crate::model::explosion::default::DefaultExplosion;
use crate::model::explosion::Explosion;
use crate::world::default::DefaultWorld;
use crate::world::World;
use com_croftsoft_core::math::geom::circle::Circle;
use core::cell::Cell;
use std::rc::Rc;

#[derive(Default)]
pub struct DefaultWorldFactory {
  // TODO: maybe use an atomic instead of Cell for interior mutability
  id_next_bullet: Cell<usize>,
  id_next_explosion: Cell<usize>,
}

impl WorldFactory for DefaultWorldFactory {
  fn make_bullet(
    &self,
    heading: f64,
    origin_x: f64,
    origin_y: f64,
  ) -> Box<dyn Bullet> {
    let id = self.id_next_bullet.get();
    self.id_next_bullet.set(id + 1);
    let bullet = DefaultBullet::new(heading, id, origin_x, origin_y);
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

  fn make_world(&self) -> Rc<dyn World> {
    Rc::new(DefaultWorld::default())
  }
}
