// =============================================================================
//! - Tank state for CroftSoft Mars
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

use super::TankAccessor;
use crate::constants::TANK_Z;
use crate::engine::traits::{Color, ModelAccessor};
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_core::math::geom::point_2dd::Point2DD;
// use com_croftsoft_core::math::geom::point_xy::PointXY;

pub struct TankState {
  active: bool,
  ammo: usize,
  // ammo_dumps: Vec<AmmoDump>,
  body_heading: f64,
  circle: Circle,
  // color: dyn Color,
  damage: f64,
  // destination: dyn PointXY,
  dry_firing: bool,
  firing: bool,
  sparking: bool,
  // tank_operator: dyn TankOperator,
  // target_point: Point2DD,
  // test_circle: Circle,
  turret_heading: f64,
  updated: bool,
  // world: World,
}

impl ModelAccessor for TankState {
  fn get_shape(&self) -> Box<dyn crate::engine::traits::Shape> {
    todo!()
  }

  fn get_z(&self) -> f64 {
    TANK_Z
  }

  fn is_active(&self) -> bool {
    self.active
  }

  fn is_updated(&self) -> bool {
    self.updated
  }
}

impl TankAccessor for TankState {
  fn get_ammo(&self) -> usize {
    self.ammo
  }

  fn get_body_heading(&self) -> f64 {
    self.body_heading
  }

  fn get_color(
    &self,
    color: Color,
  ) -> Color {
    // TODO
    color
  }

  fn get_damage(&self) -> f64 {
    self.damage
  }

  fn get_radius(&self) -> f64 {
    self.circle.radius
  }

  fn get_turret_heading(&self) -> f64 {
    self.turret_heading
  }

  fn is_dry_firing(&self) -> bool {
    self.dry_firing
  }

  fn is_firing(&self) -> bool {
    self.firing
  }

  fn is_sparking(&self) -> bool {
    self.sparking
  }
}
