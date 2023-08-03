// =============================================================================
//! - Tank traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-29
//! - Updated: 2023-08-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::{Damageable, Model, ModelAccessor};
use crate::ai::tank_operator::TankOperator;
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_core::math::geom::point_2dd::Point2DD;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub mod default;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
  // TODO: Support more than two colors
  BLUE,
  RED,
}

// trait TankConsole
pub trait Tank: Damageable + Model + TankAccessor {
  // TODO: remove this mutator methods by moving TankOperator into Tank update
  //   or by having TankOperator send commands via an input queue

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

pub trait TankAccessor: ModelAccessor {
  fn get_ammo(&self) -> usize;
  fn get_body_heading(&self) -> f64;
  fn get_body_rotation_speed(&self) -> f64;
  fn get_center(&self) -> Point2DD;
  // TODO: Move this out of TankAccessor to World or Cartographer
  // TODO: was PointXY
  fn get_closest_ammo_dump_circle(&self) -> Option<Circle>;
  // TODO: Move this out of TankAccessor to World or Cartographer
  fn get_closest_enemy_tank_circle(
    &self,
    tank_operators: Rc<RefCell<VecDeque<Box<dyn TankOperator>>>>,
  ) -> Option<Circle>;
  fn get_color(&self) -> Color;
  fn get_damage(&self) -> f64;
  fn get_radius(&self) -> f64;
  fn get_tank_speed(&self) -> f64;
  fn get_tread_offset_left(&self) -> f64;
  fn get_tread_offset_right(&self) -> f64;
  fn get_turret_heading(&self) -> f64;
  fn is_burning(&self) -> bool;
  fn is_dry_firing(&self) -> bool;
  fn is_firing(&self) -> bool;
  fn is_sparking(&self) -> bool;
}
