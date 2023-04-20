// =============================================================================
//! - Artificial Intelligence for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-07
//! - Updated: 2023-04-19
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::SpaceTester;
use crate::models::tank::state::TankState;
use crate::models::tank::TankAccessor;
use com_croftsoft_core::math::geom::point_2dd::Point2DD;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub trait TankConsole: TankAccessor + SpaceTester {
  fn fire(&mut self);
  fn get_body_rotation_speed(&self) -> f64;
  // TODO: was get_shape
  fn get_center(
    &self,
    center: &mut Point2DD,
  );
  // TODO: was PointXY
  fn get_closest_ammo_dump_center(&self) -> Option<Point2DD>;
  // TODO: was PointXY
  fn get_closest_enemy_tank_center(
    &self,
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
  ) -> Option<Point2DD>;
  fn get_tank_speed(&self) -> f64;
  fn go(
    &mut self,
    // TODO: was PointXY
    destination: &Point2DD,
  );
  // TODO: was targetPointXY
  fn rotate_turret(
    &mut self,
    target_point: &Option<Point2DD>,
  );
}
