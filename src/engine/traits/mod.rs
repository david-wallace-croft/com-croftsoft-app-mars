// =============================================================================
//! - Traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-03-23
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_lib_role::{Initializer, Updater};

pub trait Color {
  // TODO: java.awt.Color
}

pub trait Component: Initializer + Updater {
  fn make_html(&self) -> String;
}

pub trait Damageable: Model {
  fn add_damage(
    &mut self,
    damage: f64,
  );
}

pub trait Impassable: Model {}

pub trait Model: ModelAccessor {
  fn prepare(&mut self);
  fn set_center(
    &mut self,
    x: f64,
    y: f64,
  );
  fn update(
    &mut self,
    time_delta: f64,
  );
}

pub trait ModelAccessor {
  fn get_shape(&self) -> dyn Shape;
  fn get_z(&self) -> f64;
  fn is_active(&self) -> bool;
  fn is_updated(&self) -> bool;
}

pub trait Obstacle: Damageable + Impassable + ObstacleAccessor {
  fn set_active(
    &mut self,
    active: f64,
  );
  fn set_radius(
    &mut self,
    radius: f64,
  );
}

pub trait ObstacleAccessor: ModelAccessor {}

pub trait Shape {
  // TODO: java.awt.Shape
}

pub trait SpaceTester {
  fn is_space_available(
    &self,
    // TODO: this was PointXY
    x: f64,
    y: f64,
  ) -> bool;
}

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
  fn get_color(&self) -> dyn Color;
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
