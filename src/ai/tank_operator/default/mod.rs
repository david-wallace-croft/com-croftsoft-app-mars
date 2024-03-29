// =============================================================================
//! - Default Tank Operater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-06
//! - Updated: 2023-08-14
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::ai::state_space_node::StateSpaceNode;
use crate::ai::tank_cartographer::TankCartographer;
use crate::ai::tank_operator::TankOperator;
use crate::constant::{
  A_STAR_DIRECTIONS, A_STAR_LOOPS, A_STAR_STEP_SIZE, TANK_DRIFT_PROBABILITY,
  TANK_FIRING_PROBABILITY,
};
use crate::model::tank::Tank;
use crate::world::World;
use com_croftsoft_core::ai::astar::structures::AStar;
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use com_croftsoft_core::math::geom::point_2dd::Point2DD;
use core::cell::{RefCell, RefMut};
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::rngs::ThreadRng;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

pub struct DefaultTankOperator {
  a_star: AStar<StateSpaceNode>,
  center: Point2DD,
  destination: Point2DD,
  id: usize,
  start_state_space_node: StateSpaceNode,
  tank: Rc<RefCell<dyn Tank>>,
  tank_cartographer: TankCartographer,
  target_circle: Option<Circle>,
  world: Weak<dyn World>,
}

impl DefaultTankOperator {
  fn get_first_step(
    &mut self,
    destination: Circle,
    heading: f64,
  ) -> Point2DD {
    self.start_state_space_node.set_point_xy(&self.center);
    self.start_state_space_node.set_heading(heading);
    self.a_star.reset(self.start_state_space_node);
    self.tank_cartographer.reset();
    // TODO: How was this passed in the original code?
    self
      .tank_cartographer
      .set_start_state_space_node(self.start_state_space_node);
    self.tank_cartographer.set_goal_circle(destination);
    for _ in 0..A_STAR_LOOPS {
      if !self.a_star.loop_once(&self.tank_cartographer) {
        break;
      }
    }
    if !self.a_star.is_goal_found() {
      return destination.get_center_point_2dd();
    }
    if let Some(state_space_node) = self.a_star.get_first_step() {
      state_space_node.get_point_xy()
    } else {
      destination.get_center_point_2dd()
    }
  }

  pub fn new(
    id: usize,
    tank: Rc<RefCell<dyn Tank>>,
    world: Weak<dyn World>,
  ) -> Self {
    let tank_cartographer = TankCartographer::new(
      id,
      A_STAR_STEP_SIZE,
      A_STAR_DIRECTIONS,
      Rc::downgrade(&tank),
      world.clone(),
    );
    let a_star = AStar::<StateSpaceNode>::default();
    let center = Point2DD::default();
    let destination = Point2DD::default();
    let target_circle = None;
    let start_state_space_node = StateSpaceNode::default();
    Self {
      a_star,
      center,
      destination,
      id,
      start_state_space_node,
      tank_cartographer,
      tank,
      target_circle,
      world,
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

  fn get_nodes(&self) -> Vec<StateSpaceNode> {
    let mut nodes = Vec::<StateSpaceNode>::new();
    for node in self.a_star.node_to_node_info_map.keys() {
      nodes.push(*node);
    }
    nodes
  }

  // TODO: was iterator
  fn get_path(&self) -> VecDeque<StateSpaceNode> {
    self.a_star.get_path()
  }

  fn get_tank(&self) -> Rc<RefCell<dyn Tank>> {
    self.tank.clone()
  }

  fn update(
    &mut self,
    time_delta: f64,
  ) {
    let tank: Rc<RefCell<dyn Tank>> = self.tank.clone();
    {
      // Rotate turret toward nearest enemy tank
      let mut tank: RefMut<dyn Tank> = tank.borrow_mut();
      self.center = tank.get_center();
      self.target_circle = tank.get_closest_enemy_tank_circle(
        self.world.upgrade().unwrap().get_tank_operators(),
      );
      if self.target_circle.is_none() {
        // Rotate turret toward nearest obstacle
        self.target_circle = self
          .world
          .upgrade()
          .unwrap()
          .get_closest_obstacle_circle(&self.center);
      }
      let target_point: Option<Point2DD> = self
        .target_circle
        .as_ref()
        .map(|target_circle| target_circle.get_center_point_2dd());
      tank.rotate_turret(&target_point);
    }
    {
      // Move toward nearest ammo dump
      let ammo: usize = tank.borrow().get_ammo();
      if ammo < 1 {
        let closest_ammo_dump_circle_option: Option<Circle> =
          tank.borrow().get_closest_ammo_dump_circle();
        if let Some(closest_ammo_dump_circle) = closest_ammo_dump_circle_option
        {
          let destination: Point2DD = self.get_first_step(
            closest_ammo_dump_circle,
            tank.borrow().get_body_heading(),
          );
          tank.borrow_mut().go(&destination);
        }
        return;
      }
    }
    // Move toward nearest enemy tank
    let mut thread_rng: ThreadRng = rand::thread_rng();
    let uniform = Uniform::from(0.0..1.);
    if let Some(enemy_circle) = self.target_circle {
      let destination: Point2DD =
        self.get_first_step(enemy_circle, tank.borrow().get_body_heading());
      tank.borrow_mut().go(&destination);
    } else {
      // Move randomly
      let random_number = uniform.sample(&mut thread_rng);
      if random_number < time_delta * TANK_DRIFT_PROBABILITY {
        let uniform_drift = Uniform::from(-1.0..=1.0);
        let drift_x = uniform_drift.sample(&mut thread_rng);
        let drift_y = uniform_drift.sample(&mut thread_rng);
        let destination_x = self.center.x + drift_x;
        let destination_y = self.center.y + drift_y;
        self.destination.set_xy(destination_x, destination_y);
        tank.borrow_mut().go(&self.destination);
      }
      // Clears the node animation
      // TODO: Is this the best way to do this?
      self.start_state_space_node.set_point_xy(&self.center);
      self.a_star.reset(self.start_state_space_node);
    }
    // Fire randomly
    let random_number = uniform.sample(&mut thread_rng);
    if random_number < time_delta * TANK_FIRING_PROBABILITY {
      tank.borrow_mut().fire();
    }
  }
}
