// =============================================================================
//! - State Space Node for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-07
//! - Updated: 2023-04-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_core::math::geom::point_2dd::Point2DD;

pub struct StateSpaceNode {
  heading: f64,
  point_2dd: Point2DD,
}

impl StateSpaceNode {
  pub fn distance(&self, other_state_space_node: StateSpaceNode) -> f64 {
    todo!();
  }

  pub fn get_heading(&self) -> f64 {
    self.heading
  }

  pub fn get_point_xy(&self) -> Point2DD {
    self.point_2dd
  }

  pub fn new(heading: f64, point_xy: Point2DD) -> Self {
    Self {
      heading,
      point_2dd: point_xy,
    }
  }

  pub fn rotation(&self, other_state_space_node: StateSpaceNode) -> f64 {
    todo!();
  }

  pub fn set(&self, state_space_node: &mut Self) {
    state_space_node.set_heading(self.heading);
    state_space_node.set_point_xy(self.point_2dd);
  }

  pub fn set_heading(&mut self, heading: f64) {
    todo!();
  }

  pub fn set_point_xy(&mut self, point_xy: Point2DD) {
    todo!();
  }

  pub fn to_string(&self) -> String {
    todo!();
  }
}
