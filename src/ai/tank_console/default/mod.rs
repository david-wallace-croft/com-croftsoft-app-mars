// =============================================================================
//! - Default Tank Console for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-25
//! - Updated: 2023-05-24
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::TankConsole;
use crate::engine::traits::{ModelAccessor, SpaceTester};
use crate::models::tank::Tank;
use crate::models::world::World;
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use com_croftsoft_core::math::geom::point_2dd::Point2DD;
use core::cell::RefCell;
use core::f64::INFINITY;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct DefaultTankConsole {
  pub tank: Rc<RefCell<dyn Tank>>,
  pub world: Rc<dyn World>,
}

impl ModelAccessor for DefaultTankConsole {
  fn contains(
    &self,
    x: f64,
    y: f64,
  ) -> bool {
    self.tank.borrow().contains(x, y)
  }

  fn get_circle(&self) -> Circle {
    self.tank.borrow().get_circle()
  }

  fn get_id(&self) -> usize {
    self.tank.borrow().get_id()
  }

  fn get_z(&self) -> f64 {
    self.tank.borrow().get_z()
  }

  fn intersects_circle(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool {
    self.tank.borrow().intersects_circle(circle)
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
    // TODO: this was PointXY; could be a Circle
    x: f64,
    y: f64,
  ) -> bool {
    let self_tank = self.tank.borrow();
    let mut tank_circle = self_tank.get_circle();
    tank_circle.center_x = x;
    tank_circle.center_y = y;
    // TODO: previously operated on an array of Impassable
    for obstacle in self.world.get_obstacles().borrow().iter() {
      if obstacle.circle.intersects_circle(&tank_circle) {
        return false;
      }
    }
    let self_tank_color = self_tank.get_color();
    for other_tank in self.world.get_tanks().borrow().iter() {
      let other_tank = other_tank.borrow();
      if self_tank_color != other_tank.get_color() && self_tank.get_ammo() > 0 {
        continue;
      }
      let other_tank_circle = other_tank.get_circle();
      if other_tank_circle.intersects_circle(&tank_circle) {
        return false;
      }
    }
    true
  }
}

impl TankConsole for DefaultTankConsole {
  fn fire(&mut self) {
    self.tank.borrow_mut().fire();
  }

  fn get_ammo(&self) -> usize {
    self.tank.borrow().get_ammo()
  }

  fn get_body_heading(&self) -> f64 {
    self.tank.borrow().get_body_heading()
  }

  fn get_body_rotation_speed(&self) -> f64 {
    self.tank.borrow().get_body_rotation_speed()
  }

  fn get_center(&self) -> Point2DD {
    self.tank.borrow().get_center()
  }

  fn get_closest_ammo_dump_center(&self) -> Option<Point2DD> {
    let mut closest_ammo_dump_center: Option<Point2DD> = None;
    let tank_center = self.tank.borrow().get_center();
    let mut closest_distance: f64 = INFINITY;
    let world = &self.world;
    let ammo_dumps = world.get_ammo_dumps();
    for ammo_dump in ammo_dumps.borrow().iter() {
      let ammo_dump_center = ammo_dump.get_circle().get_center_point_2dd();
      let distance: f64 = tank_center.distance_to(&ammo_dump_center);
      if distance < closest_distance {
        closest_distance = distance;
        closest_ammo_dump_center = Some(ammo_dump_center);
      }
    }
    closest_ammo_dump_center
  }

  fn get_closest_enemy_tank_center(
    &self,
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<dyn Tank>>>>>,
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
