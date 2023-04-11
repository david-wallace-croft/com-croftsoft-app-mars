// =============================================================================
//! - State Space Node for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-07
//! - Updated: 2023-04-11
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_core::math::geom::point_2dd::Point2DD;
use core::hash::Hash;
use std::{
  f64::consts::{PI, TAU},
  fmt::Display,
};

#[derive(Default)]
pub struct StateSpaceNode {
  heading: f64,
  point_2dd: Point2DD,
}

impl StateSpaceNode {
  pub fn distance(
    &self,
    other_state_space_node: &StateSpaceNode,
  ) -> f64 {
    self.point_2dd.distance_to(&other_state_space_node.point_2dd)
  }

  pub fn get_heading(&self) -> f64 {
    self.heading
  }

  pub fn get_point_xy(&self) -> Point2DD {
    self.point_2dd
  }

  pub fn new(
    heading: f64,
    point_xy: Point2DD,
  ) -> Self {
    Self {
      heading,
      point_2dd: point_xy,
    }
  }

  pub fn rotation(
    &self,
    other_state_space_node: &StateSpaceNode,
  ) -> f64 {
    // note from old code: this needs to be fixed
    let other_heading: f64 = other_state_space_node.heading;
    let mut heading_delta: f64 = other_heading - self.heading;
    if heading_delta < -PI {
      heading_delta = (other_heading + TAU) - self.heading;
    } else if heading_delta > PI {
      heading_delta = (other_heading - TAU) - self.heading;
    }
    heading_delta
  }

  pub fn set(
    &mut self,
    state_space_node: StateSpaceNode,
  ) {
    self.set_heading(state_space_node.get_heading());
    self.set_point_xy(&state_space_node.get_point_xy());
  }

  pub fn set_heading(
    &mut self,
    mut heading: f64,
  ) {
    while heading < 0. {
      heading += TAU;
    }
    while heading > TAU {
      heading -= TAU;
    }
    self.heading = heading;
  }

  pub fn set_point_xy(
    &mut self,
    point_xy: &Point2DD,
  ) {
    self.point_2dd.set_xy_point(point_xy);
  }
}

impl Display for StateSpaceNode {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{}", self.point_2dd)
  }
}

impl Eq for StateSpaceNode {}

impl Hash for StateSpaceNode {
  fn hash<H: std::hash::Hasher>(
    &self,
    state: &mut H,
  ) {
    self.point_2dd.hash(state);
  }
}

impl PartialEq for StateSpaceNode {
  fn eq(
    &self,
    other: &Self,
  ) -> bool {
    self.point_2dd == other.point_2dd
  }
}
