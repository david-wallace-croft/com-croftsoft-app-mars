// =============================================================================
//! - World trait for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-20
//! - Updated: 2023-05-20
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use self::factory::WorldFactory;
use super::ammo_dump::default::DefaultAmmoDump;
use super::bullet::Bullet;
use super::explosion::Explosion;
use super::obstacle::state::ObstacleState;
use super::tank::state::TankState;
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
pub mod updater;

pub trait World {
  fn add_bullet(
    &self,
    bullet: Box<dyn Bullet>,
  );

  fn add_explosion(
    &self,
    explosion: Box<dyn Explosion>,
  );

  // TODO: Use the AmmoDump trait
  fn get_ammo_dumps(&self) -> Rc<RefCell<VecDeque<DefaultAmmoDump>>>;

  fn get_bullets(&self) -> Rc<RefCell<VecDeque<Box<dyn Bullet>>>>;

  fn get_explosions(&self) -> Rc<RefCell<VecDeque<Box<dyn Explosion>>>>;

  // TODO: move WorldFactory out of World
  fn get_factory(&self) -> Rc<dyn WorldFactory>;

  fn get_obstacles(&self) -> Rc<RefCell<VecDeque<ObstacleState>>>;

  fn get_tank_operators(
    &self
  ) -> Rc<RefCell<VecDeque<Rc<RefCell<dyn TankOperator>>>>>;

  fn get_tanks(&self) -> Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>;

  fn is_blocked_by_impassable(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool;
}
