// =============================================================================
//! - Default World Factory for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-05-17
//! - Updated: 2023-05-19
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::WorldFactory;
use crate::models::explosion::default::DefaultExplosion;
use crate::models::explosion::Explosion;
use com_croftsoft_core::math::geom::circle::Circle;
use core::cell::Cell;

#[derive(Default)]
pub struct DefaultWorldFactory {
  id_next_explosion: Cell<usize>,
}

impl WorldFactory for DefaultWorldFactory {
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
}
