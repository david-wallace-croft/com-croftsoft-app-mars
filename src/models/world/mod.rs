// =============================================================================
//! - World trait for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-20
//! - Updated: 2023-06-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::ammo_dump::AmmoDump;
use super::bullet::Bullet;
use super::explosion::Explosion;
use super::obstacle::Obstacle;
use super::tank::Tank;
use super::tank_operator::TankOperator;
use com_croftsoft_core::math::geom::circle::CircleAccessor;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub mod builder;
pub mod default;
pub mod director;
pub mod factory;
pub mod preparer;
pub mod seed;

pub trait World {
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

  fn add_tank(
    &self,
    tank: Rc<RefCell<dyn Tank>>,
  );

  fn add_tank_operator(
    &self,
    tank_operator: Rc<RefCell<dyn TankOperator>>,
  );

  fn compute_bullet_damage(
    &self,
    circle: &dyn CircleAccessor,
  ) -> f64;

  fn compute_explosion_damage(
    &self,
    circle: &dyn CircleAccessor,
  ) -> f64;

  fn get_ammo_dumps(&self) -> Rc<RefCell<VecDeque<Box<dyn AmmoDump>>>>;

  fn get_bullets(&self) -> Rc<RefCell<VecDeque<Box<dyn Bullet>>>>;

  fn get_explosions(&self) -> Rc<RefCell<VecDeque<Box<dyn Explosion>>>>;

  fn get_obstacles(&self) -> Rc<RefCell<VecDeque<Box<dyn Obstacle>>>>;

  fn get_tank_operators(
    &self
  ) -> Rc<RefCell<VecDeque<Rc<RefCell<dyn TankOperator>>>>>;

  fn get_tanks(&self) -> Rc<RefCell<VecDeque<Rc<RefCell<dyn Tank>>>>>;

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
