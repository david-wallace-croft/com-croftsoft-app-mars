// =============================================================================
//! - Tank traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-29
//! - Updated: 2023-03-29
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::{
  Color, Damageable, Impassable, Model, ModelAccessor, Shape, SpaceTester,
};

pub mod state;

pub trait Tank: Damageable + Impassable + Model + TankConsole {
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
  fn get_closest_ammo_dump_center(&self) -> (f64, f64); // TODO: was PointXY
  fn get_closest_enemy_tank_center(&self) -> (f64, f64); // TODO: was PointXY
  fn get_shape(&self) -> dyn Shape;
  fn get_tank_speed(&self) -> f64;
  fn go(
    &mut self,
    x: f64,
    y: f64,
  ); // TODO: was PointXY destination
  fn rotate_turret(
    &mut self,
    x: f64,
    y: f64,
  ); // TODO: was targetPointXY
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
