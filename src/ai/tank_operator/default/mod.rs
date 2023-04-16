// =============================================================================
//! - Default Tank Operater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-06
//! - Updated: 2023-04-16
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::TankOperator;
use crate::ai::state_space_node::StateSpaceNode;
use crate::ai::tank_cartographer::TankCartographer;
use crate::ai::tank_console::TankConsole;
use crate::constants::{
  A_STAR_DIRECTIONS, A_STAR_LOOPS, A_STAR_STEP_SIZE, TANK_DRIFT_PROBABILITY,
  TANK_FIRING_PROBABILITY,
};
use com_croftsoft_core::ai::astar::structures::AStar;
use com_croftsoft_core::math::geom::point_2dd::Point2DD;
use core::cell::{RefCell, RefMut};
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::rngs::ThreadRng;
use std::rc::Rc;

pub struct DefaultTankOperator {
  a_star: AStar<TankCartographer, StateSpaceNode>,
  center: Point2DD,
  destination: Point2DD,
  // TODO: was PointXY
  enemy_center: Option<Point2DD>,
  start_state_space_node: StateSpaceNode,
  tank_cartographer: Rc<RefCell<TankCartographer>>,
  tank_console: Option<Rc<RefCell<dyn TankConsole>>>,
}

impl DefaultTankOperator {
  fn get_first_step(
    &mut self,
    destination: &Point2DD,
  ) -> Point2DD {
    self.start_state_space_node.set_point_xy(&self.center);
    self.start_state_space_node.set_heading(
      self.tank_console.as_ref().unwrap().borrow().get_body_heading(),
    );
    self.a_star.reset(self.start_state_space_node);
    self.tank_cartographer.borrow_mut().set_goal_point_xy(destination);
    for _ in 0..A_STAR_LOOPS {
      if !self.a_star.loop_once() {
        break;
      }
    }
    if !self.a_star.is_goal_found() {
      return *destination;
    }
    if let Some(state_space_node) = self.a_star.get_first_step() {
      state_space_node.get_point_xy()
    } else {
      *destination
    }
  }
}

impl Default for DefaultTankOperator {
  fn default() -> Self {
    let tank_cartographer = Rc::new(RefCell::new(TankCartographer::new(
      A_STAR_STEP_SIZE,
      A_STAR_DIRECTIONS,
    )));
    let a_star = AStar::new(tank_cartographer.clone());
    let center = Point2DD::default();
    let destination = Point2DD::default();
    let enemy_center = None;
    let start_state_space_node = StateSpaceNode::default();
    let tank_console = None;
    Self {
      a_star,
      center,
      destination,
      enemy_center,
      start_state_space_node,
      tank_cartographer,
      tank_console,
    }
  }
}

impl TankOperator for DefaultTankOperator {
  fn fire(&mut self) {
    // was empty in the old code
  }

  // TODO: was iterator
  fn get_path(&self) -> Vec<StateSpaceNode> {
    self.a_star.get_path()
  }

  fn set_tank_console(
    &mut self,
    tank_console: Rc<RefCell<dyn TankConsole>>,
  ) {
    // TODO: old code would delegate to the existing tank cartographer
    let tank_cartographer = Rc::new(RefCell::new(TankCartographer::new(
      A_STAR_STEP_SIZE,
      A_STAR_DIRECTIONS,
    )));
    tank_cartographer.borrow_mut().set_tank_console(tank_console.clone());
    self.a_star = AStar::new(tank_cartographer);
    self.tank_console = Some(tank_console);
  }

  fn update(
    &mut self,
    time_delta: f64,
  ) {
    let Some(tank_console) = &self.tank_console else { return; };
    let tank_console: Rc<RefCell<dyn TankConsole>> = tank_console.clone();
    let mut tank_console: RefMut<dyn TankConsole> = tank_console.borrow_mut();
    tank_console.get_center(&mut self.center);
    self.enemy_center = tank_console.get_closest_enemy_tank_center();
    tank_console.rotate_turret(&self.enemy_center);
    if tank_console.get_ammo() < 1 {
      let closest_ammo_dump_center_option: Option<Point2DD> =
        tank_console.get_closest_ammo_dump_center();
      if let Some(closest_ammo_dump_center) = closest_ammo_dump_center_option {
        let destination: Point2DD =
          self.get_first_step(&closest_ammo_dump_center);
        tank_console.go(&destination);
      }
      return;
    }
    let mut thread_rng: ThreadRng = rand::thread_rng();
    let uniform = Uniform::from(0.0..1.);
    if let Some(enemy_center) = self.enemy_center {
      let destination: Point2DD = self.get_first_step(&enemy_center);
      tank_console.go(&destination);
      let random_number = uniform.sample(&mut thread_rng);
      if random_number < time_delta * TANK_FIRING_PROBABILITY {
        tank_console.fire();
      }
      return;
    }
    let random_number = uniform.sample(&mut thread_rng);
    if random_number < time_delta * TANK_DRIFT_PROBABILITY {
      let uniform_drift = Uniform::from(-1.0..=1.0);
      let drift_x = uniform_drift.sample(&mut thread_rng);
      let drift_y = uniform_drift.sample(&mut thread_rng);
      let destination_x = self.center.x + drift_x;
      let destination_y = self.center.y + drift_y;
      self.destination.set_xy(destination_x, destination_y);
      tank_console.go(&self.destination);
    }
    let random_number = uniform.sample(&mut thread_rng);
    if random_number < time_delta * TANK_FIRING_PROBABILITY {
      tank_console.fire();
    }
  }
}