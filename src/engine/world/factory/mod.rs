// =============================================================================
//! - World Factory trait for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-05-17
//! - Updated: 2023-06-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::World;
use crate::ai::tank_operator::TankOperator;
use crate::model::bullet::Bullet;
use crate::model::explosion::Explosion;
use crate::model::tank::Tank;
use com_croftsoft_core::math::geom::circle::Circle;
use core::cell::RefCell;
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

  fn make_tank_operator(
    &self,
    tank: Rc<RefCell<dyn Tank>>,
  ) -> Rc<RefCell<dyn TankOperator>>;

  fn make_world(&self) -> Rc<dyn World>;
}
