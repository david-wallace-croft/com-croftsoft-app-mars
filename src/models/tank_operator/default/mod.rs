// =============================================================================
//! - Default Tank Operater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-06
//! - Updated: 2023-05-20
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::ai::state_space_node::StateSpaceNode;
use crate::ai::tank_cartographer::TankCartographer;
use crate::ai::tank_console::TankConsole;
use crate::constants::{
  A_STAR_DIRECTIONS, A_STAR_LOOPS, A_STAR_STEP_SIZE, TANK_DRIFT_PROBABILITY,
  TANK_FIRING_PROBABILITY,
};
use crate::models::tank::state::TankState;
use crate::models::tank_operator::TankOperator;
use com_croftsoft_core::ai::astar::structures::AStar;
use com_croftsoft_core::math::geom::point_2dd::Point2DD;
use core::cell::{RefCell, RefMut};
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::rngs::ThreadRng;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct DefaultTankOperator {
  a_star: AStar<TankCartographer, StateSpaceNode>,
  center: Point2DD,
  destination: Point2DD,
  // TODO: was PointXY
  enemy_center: Option<Point2DD>,
  id: usize,
  start_state_space_node: StateSpaceNode,
  tank_cartographer: Rc<RefCell<TankCartographer>>,
  tank_console: Option<Rc<RefCell<dyn TankConsole>>>,
}

impl DefaultTankOperator {
  fn get_first_step(
    &mut self,
    destination: &Point2DD,
    heading: f64,
  ) -> Point2DD {
    self.start_state_space_node.set_point_xy(&self.center);
    self.start_state_space_node.set_heading(heading);
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

  pub fn new(id: usize) -> Self {
    let tank_cartographer = Rc::new(RefCell::new(TankCartographer::new(
      id,
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
      id,
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

  fn get_id(&self) -> usize {
    self.id
  }

  // TODO: was iterator
  fn get_path(&self) -> Vec<StateSpaceNode> {
    self.a_star.get_path()
  }

  fn set_tank_console(
    &mut self,
    tank_console: Rc<RefCell<dyn TankConsole>>,
  ) {
    self.tank_cartographer.borrow_mut().set_tank_console(tank_console.clone());
    self.a_star = AStar::new(self.tank_cartographer.clone());
    self.tank_console = Some(tank_console);
  }

  fn update(
    &mut self,
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
    time_delta: f64,
  ) {
    // log("DefaultTankOperator.update");
    let Some(tank_console) = &self.tank_console else { return; };
    let tank_console: Rc<RefCell<dyn TankConsole>> = tank_console.clone();
    {
      let mut tank_console: RefMut<dyn TankConsole> = tank_console.borrow_mut();
      tank_console.get_center(&mut self.center);
      self.enemy_center = tank_console.get_closest_enemy_tank_center(tanks);
      tank_console.rotate_turret(&self.enemy_center);
    }
    {
      let ammo: usize = tank_console.borrow().get_ammo();
      if ammo < 1 {
        let closest_ammo_dump_center_option: Option<Point2DD> =
          tank_console.borrow().get_closest_ammo_dump_center();
        if let Some(closest_ammo_dump_center) = closest_ammo_dump_center_option
        {
          let destination: Point2DD = self.get_first_step(
            &closest_ammo_dump_center,
            tank_console.borrow().get_body_heading(),
          );
          tank_console.borrow_mut().go(&destination);
        }
        return;
      }
    }
    let mut thread_rng: ThreadRng = rand::thread_rng();
    let uniform = Uniform::from(0.0..1.);
    if let Some(enemy_center) = self.enemy_center {
      let destination: Point2DD = self.get_first_step(
        &enemy_center,
        tank_console.borrow().get_body_heading(),
      );
      tank_console.borrow_mut().go(&destination);
      let random_number = uniform.sample(&mut thread_rng);
      if random_number < time_delta * TANK_FIRING_PROBABILITY {
        tank_console.borrow_mut().fire();
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
      tank_console.borrow_mut().go(&self.destination);
    }
    let random_number = uniform.sample(&mut thread_rng);
    if random_number < time_delta * TANK_FIRING_PROBABILITY {
      tank_console.borrow_mut().fire();
    }
  }
}