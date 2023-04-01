// =============================================================================
//! - Tank traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-29
//! - Updated: 2023-04-01
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::{
  Color, Damageable, Impassable, Model, ModelAccessor, SpaceTester,
};
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_core::math::geom::point_2dd::Point2DD;

pub mod state;

pub trait Tank:
  Damageable + Impassable + Model + TankConsole + TankMutator
{
  fn get_tank_operator(&self) -> dyn TankOperator;
  fn initialize(
    &mut self,
    center_x: f64,
    center_y: f64,
  );
  fn set_tank_operator(
    &mut self,
    tank_operator: dyn TankOperator,
  );
}

pub trait TankAccessor: ModelAccessor {
  fn get_ammo(&self) -> usize;
  fn get_body_heading(&self) -> f64;
  fn get_color(
    &self,
    color: Color,
  ) -> Color;
  fn get_damage(&self) -> f64;
  fn get_radius(&self) -> f64;
  fn get_turret_heading(&self) -> f64;
  fn is_dry_firing(&self) -> bool;
  fn is_firing(&self) -> bool;
  fn is_sparking(&self) -> bool;
}

pub trait TankConsole: TankAccessor + SpaceTester {
  fn fire(&mut self);
  fn get_body_rotation_speed(&self) -> f64;
  // TODO: was PointXY
  fn get_closest_ammo_dump_center(&self) -> (f64, f64);
  // TODO: was PointXY
  fn get_closest_enemy_tank_center(&self) -> (f64, f64);
  // TODO: have this return Shape
  fn get_shape(
    &self,
    circle: Circle,
  ) -> Circle;
  fn get_tank_speed(&self) -> f64;
  fn go(
    &mut self,
    // TODO: was PointXY
    destination: Option<Point2DD>,
  );
  // TODO: was targetPointXY
  fn rotate_turret(
    &mut self,
    target_point: Point2DD,
  );
}

pub trait TankOperator {
  fn fire(&mut self);
  fn get_path(&self) -> Vec<(f64, f64)>; // TODO: was iterator
  fn set_tank_console(
    &mut self,
    tank_console: dyn TankConsole,
  );
  fn update(
    &mut self,
    time_delta: f64,
  );
}

pub trait TankMutator {
  fn set_body_heading(
    &mut self,
    body_heading: f64,
  );
}
