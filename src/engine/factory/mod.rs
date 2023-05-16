// =============================================================================
//! - Factory for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-05-16
//! - Updated: 2023-05-16
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::models::explosion::default::DefaultExplosion;
use crate::models::explosion::Explosion;
use com_croftsoft_core::math::geom::circle::Circle;

#[derive(Default)]
pub struct Factory {
  id_next_explosion: usize,
}

impl Factory {
  pub fn make_explosion(
    &mut self,
    circle: Circle,
    damage: f64,
  ) -> Box<dyn Explosion> {
    let explosion =
      DefaultExplosion::new(circle, damage, self.id_next_explosion);
    Box::new(explosion)
  }
}
