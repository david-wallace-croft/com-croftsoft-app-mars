// =============================================================================
//! - Tank traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-29
//! - Updated: 2023-05-28
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::{
  Color, Impassable, Model, ModelAccessor, SpaceTester,
};
use com_croftsoft_core::math::geom::point_2dd::Point2DD;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub mod default;
pub mod preparer;
pub mod updater;

// trait TankConsole
pub trait Tank:
  Impassable + Model + SpaceTester + TankAccessor + TankMutator
{
  // fn get_tank_operator(&self) -> Rc<RefCell<dyn TankOperator>>;

  // TODO: was PointXY
  fn get_closest_ammo_dump_center(&self) -> Option<Point2DD>;

  fn initialize(
    &mut self,
    center_x: f64,
    center_y: f64,
  );

  // fn set_tank_operator(
  //   &mut self,
  //   tank_operator: Rc<RefCell<dyn TankOperator>>,
  // );
}

pub trait TankAccessor: ModelAccessor {
  fn get_ammo(&self) -> usize;
  fn get_body_heading(&self) -> f64;
  fn get_body_rotation_speed(&self) -> f64;
  fn get_center(&self) -> Point2DD;
  fn get_closest_enemy_tank_center(
    &self,
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<dyn Tank>>>>>,
  ) -> Option<Point2DD>;
  fn get_color(&self) -> Color;
  fn get_damage(&self) -> f64;
  fn get_radius(&self) -> f64;
  fn get_tank_speed(&self) -> f64;
  fn get_turret_heading(&self) -> f64;
  fn is_dry_firing(&self) -> bool;
  fn is_firing(&self) -> bool;
  fn is_sparking(&self) -> bool;
}

pub trait TankMutator {
  fn fire(&mut self);

  fn go(
    &mut self,
    destination: &Point2DD,
  );

  fn rotate_turret(
    &mut self,
    target_point: &Option<Point2DD>,
  );

  fn set_ammo(
    &mut self,
    ammo: usize,
  );

  fn set_body_heading(
    &mut self,
    body_heading: f64,
  );

  fn set_turret_heading(
    &mut self,
    turret_heading: f64,
  );
}
