// =============================================================================
//! - Default Tank Console for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-25
//! - Updated: 2023-04-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::TankConsole;
use crate::engine::traits::{ModelAccessor, SpaceTester};
use crate::models::obstacle::state::ObstacleState;
use crate::models::tank::state::TankState;
use crate::models::tank::TankAccessor;
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct DefaultTankConsole {
  pub obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
  pub tank: Rc<RefCell<TankState>>,
  pub tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
}

impl ModelAccessor for DefaultTankConsole {
  fn get_shape(
    &self,
    circle: com_croftsoft_core::math::geom::circle::Circle,
  ) -> com_croftsoft_core::math::geom::circle::Circle {
    self.tank.borrow().get_shape(circle)
  }

  fn get_z(&self) -> f64 {
    self.tank.borrow().get_z()
  }

  fn is_active(&self) -> bool {
    self.tank.borrow().is_active()
  }

  fn is_updated(&self) -> bool {
    self.tank.borrow().is_updated()
  }
}

impl SpaceTester for DefaultTankConsole {
  fn is_space_available(
    &self,
    // TODO: this was PointXY
    x: f64,
    y: f64,
  ) -> bool {
    let test_circle = Circle::default();
    let mut test_circle = self.tank.borrow().get_shape(test_circle);
    test_circle.center_x = x;
    test_circle.center_y = y;
    for obstacle in self.obstacles.borrow().iter() {
      if obstacle.circle.intersects_circle(&test_circle) {
        return false;
      }
    }
    // TODO
    // let mut tank_circle = Circle::default();
    // for tank in self.tanks.borrow().iter() {
    //   tank.get_shape(tank_circle);
    //   if tank_circle.intersects_circle(&test_circle) {
    //     return false;
    //   }
    // }
    true
  }
}

impl TankAccessor for DefaultTankConsole {
  fn get_ammo(&self) -> usize {
    self.tank.borrow().get_ammo()
  }

  fn get_body_heading(&self) -> f64 {
    self.tank.borrow().get_body_heading()
  }

  fn get_color(&self) -> crate::engine::traits::Color {
    self.tank.borrow().get_color()
  }

  fn get_damage(&self) -> f64 {
    self.tank.borrow().get_damage()
  }

  fn get_radius(&self) -> f64 {
    self.tank.borrow().get_radius()
  }

  fn get_turret_heading(&self) -> f64 {
    self.tank.borrow().get_turret_heading()
  }

  fn is_dry_firing(&self) -> bool {
    self.tank.borrow().is_dry_firing()
  }

  fn is_firing(&self) -> bool {
    self.tank.borrow().is_firing()
  }

  fn is_sparking(&self) -> bool {
    self.tank.borrow().is_sparking()
  }
}

impl TankConsole for DefaultTankConsole {
  fn fire(&mut self) {
    self.tank.borrow_mut().fire();
  }

  fn get_body_rotation_speed(&self) -> f64 {
    self.tank.borrow().get_body_rotation_speed()
  }

  fn get_center(
    &self,
    center: &mut com_croftsoft_core::math::geom::point_2dd::Point2DD,
  ) {
    self.tank.borrow().get_center(center)
  }

  fn get_closest_ammo_dump_center(
    &self
  ) -> Option<com_croftsoft_core::math::geom::point_2dd::Point2DD> {
    todo!()
  }

  fn get_closest_enemy_tank_center(
    &self,
    tanks: std::rc::Rc<
      std::cell::RefCell<
        std::collections::VecDeque<
          std::rc::Rc<
            std::cell::RefCell<crate::models::tank::state::TankState>,
          >,
        >,
      >,
    >,
  ) -> Option<com_croftsoft_core::math::geom::point_2dd::Point2DD> {
    self.tank.borrow().get_closest_enemy_tank_center(tanks)
  }

  fn get_id(&self) -> usize {
    self.tank.borrow().get_id()
  }

  fn get_tank_speed(&self) -> f64 {
    self.tank.borrow().get_tank_speed()
  }

  fn go(
    &mut self,
    // TODO: was PointXY
    destination: &com_croftsoft_core::math::geom::point_2dd::Point2DD,
  ) {
    self.tank.borrow_mut().go(destination);
  }

  fn rotate_turret(
    &mut self,
    target_point: &Option<com_croftsoft_core::math::geom::point_2dd::Point2DD>,
  ) {
    self.tank.borrow_mut().rotate_turret(target_point);
  }
}
