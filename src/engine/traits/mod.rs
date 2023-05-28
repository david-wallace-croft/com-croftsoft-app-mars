// =============================================================================
//! - Traits for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-05-28
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use com_croftsoft_lib_role::{Initializer, Preparer, Updater};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
  // TODO: Support more than two colors
  BLUE,
  RED,
}

pub trait Component: Initializer + Updater {
  fn make_html(&self) -> String;
}

pub trait Impassable: Model {}

pub trait Model: ModelAccessor + Preparer {
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
  fn contains(
    &self,
    x: f64,
    y: f64,
  ) -> bool;

  fn get_circle(&self) -> Circle;

  fn get_id(&self) -> usize;

  fn get_z(&self) -> f64;

  fn intersects_circle(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool;

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
