// =============================================================================
//! - World trait for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-20
//! - Updated: 2023-06-28
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::ai::tank_operator::TankOperator;
use crate::model::ammo_dump::AmmoDump;
use crate::model::bullet::Bullet;
use crate::model::explosion::Explosion;
use crate::model::obstacle::Obstacle;
use crate::visitor::VisitorAcceptor;
use com_croftsoft_core::math::geom::circle::CircleAccessor;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub mod builder;
pub mod default;
pub mod director;
pub mod factory;
pub mod seed;

pub trait World: VisitorAcceptor {
  fn add_ammo_dump(
    &self,
    ammo_dump: Box<dyn AmmoDump>,
  );

  fn add_bullet(
    &self,
    bullet: Box<dyn Bullet>,
  );

  fn add_explosion(
    &self,
    explosion: Box<dyn Explosion>,
  );

  fn add_obstacle(
    &self,
    obstacle: Box<dyn Obstacle>,
  );

  fn add_tank_operator(
    &self,
    tank_operator: Box<dyn TankOperator>,
  );

  fn clear(&self);

  fn get_ammo_dumps(&self) -> Rc<RefCell<VecDeque<Box<dyn AmmoDump>>>>;

  fn get_bullets(&self) -> Rc<RefCell<VecDeque<Box<dyn Bullet>>>>;

  fn get_explosions(&self) -> Rc<RefCell<VecDeque<Box<dyn Explosion>>>>;

  fn get_obstacles(&self) -> Rc<RefCell<VecDeque<Box<dyn Obstacle>>>>;

  fn get_tank_operators(&self) -> Rc<RefCell<VecDeque<Box<dyn TankOperator>>>>;

  fn is_blocked_by_ammo_dump(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool;

  fn is_blocked_by_impassable(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool;

  fn is_blocked_by_tank(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool;
}
