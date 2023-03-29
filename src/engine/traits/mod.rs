// =============================================================================
//! - Traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-03-29
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_lib_role::{Initializer, Updater};

pub struct Color {
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
  fn get_shape(&self) -> Box<dyn Shape>;
  fn get_z(&self) -> f64;
  fn is_active(&self) -> bool;
  fn is_updated(&self) -> bool;
}

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
