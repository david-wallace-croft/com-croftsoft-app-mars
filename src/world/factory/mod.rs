// =============================================================================
//! - World Factory trait for CroftSoft Mars
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

use super::World;
use crate::model::bullet::Bullet;
use crate::model::explosion::Explosion;
use com_croftsoft_core::math::geom::circle::Circle;
use std::rc::Rc;

pub mod default;

pub trait WorldFactory {
  fn make_bullet(
    &self,
    heading: f64,
    origin_x: f64,
    origin_y: f64,
  ) -> Box<dyn Bullet>;

  fn make_explosion(
    &self,
    circle: Circle,
    damage: f64,
  ) -> Box<dyn Explosion>;

  fn make_world(&self) -> Rc<dyn World>;
}
