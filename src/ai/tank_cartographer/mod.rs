// =============================================================================
//! - Tank Cartographer for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-07
//! - Updated: 2023-08-01
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::state_space_node::StateSpaceNode;
use crate::model::tank::Tank;
use crate::world::World;
use com_croftsoft_core::ai::astar::traits::Cartographer;
use com_croftsoft_core::math::geom::circle::CircleAccessor;
use com_croftsoft_core::math::geom::point_2dd::Point2DD;
use com_croftsoft_core::math::geom::point_xy::PointXY;
use std::cell::{Ref, RefCell};
use std::f64::consts::PI;
use std::rc::{Rc, Weak};

pub struct TankCartographer {
  adjacent_nodes: RefCell<Vec<StateSpaceNode>>,
  directions: usize,
  goal_state_space_node: StateSpaceNode,
  id: usize,
  init_step_size: f64,
  start_state_space_node: StateSpaceNode,
  tank: Weak<RefCell<dyn Tank>>,
  world: Weak<dyn World>,
}

impl TankCartographer {
  fn calculate_travel_time(
    &self,
    from_node: &StateSpaceNode,
    to_node: &StateSpaceNode,
  ) -> f64 {
    let distance: f64 = from_node.distance(to_node);
    let tank_speed: f64 =
      self.tank.upgrade().unwrap().borrow().get_tank_speed();
    distance / tank_speed
  }

  pub fn get_id(&self) -> usize {
    self.id
  }

  // TODO: Include target radius as being available
  fn is_space_available(
    &self,
    // TODO: this was PointXY; could be a Circle
    x: f64,
    y: f64,
  ) -> bool {
    let tank: Rc<RefCell<dyn Tank>> = self.tank.upgrade().unwrap();
    let tank: Ref<dyn Tank> = tank.borrow();
    let mut tank_circle = tank.get_circle();
    tank_circle.center_x = x;
    tank_circle.center_y = y;
    // TODO: previously operated on an array of Impassable
    for obstacle in self
      .world
      .upgrade()
      .unwrap()
      .get_obstacles()
      .borrow()
      .iter()
    {
      if obstacle.get_circle().intersects_circle(&tank_circle) {
        return false;
      }
    }
    let self_tank_color = tank.get_color();
    for other_tank_operator in self
      .world
      .upgrade()
      .unwrap()
      .get_tank_operators()
      .borrow()
      .iter()
    {
      let other_tank = other_tank_operator.get_tank();
      let other_tank = other_tank.borrow();
      if !other_tank.is_active() {
        continue;
      }
      if tank.get_ammo() > 0
        && !other_tank.is_burning()
        && self_tank_color != other_tank.get_color()
      {
        continue;
      }
      let other_tank_circle = other_tank.get_circle();
      if other_tank_circle.intersects_circle(&tank_circle) {
        return false;
      }
    }
    true
  }

  pub fn new(
    id: usize,
    init_step_size: f64,
    directions: usize,
    tank: Weak<RefCell<dyn Tank>>,
    world: Weak<dyn World>,
  ) -> Self {
    let goal_state_space_node = StateSpaceNode::new(0., Point2DD::default());
    let start_state_space_node = StateSpaceNode::new(0., Point2DD::default());
    TankCartographer {
      adjacent_nodes: RefCell::new(Vec::new()),
      directions,
      goal_state_space_node,
      id,
      init_step_size,
      start_state_space_node,
      tank,
      world,
    }
  }

  pub fn push_adjacent_node(
    &self,
    adjacent_node: &StateSpaceNode,
  ) -> bool {
    let mut adjacent_nodes = self.adjacent_nodes.borrow_mut();
    for previous_node in adjacent_nodes.iter() {
      if previous_node.distance(adjacent_node) < self.init_step_size / 2. {
        // TODO: Is this incorrectly filtering out different headings?
        return false;
      }
    }
    adjacent_nodes.push(*adjacent_node);
    true
  }

  pub fn reset(&self) {
    self.adjacent_nodes.borrow_mut().clear();
  }

  pub fn set_goal_point_xy(
    &mut self,
    goal_point_xy: &Point2DD,
  ) {
    self.goal_state_space_node.set_point_xy(goal_point_xy);
  }

  pub fn set_start_state_space_node(
    &mut self,
    start_state_space_node: StateSpaceNode,
  ) {
    self.start_state_space_node.set(start_state_space_node);
  }
}

impl Cartographer<StateSpaceNode> for TankCartographer {
  fn estimate_cost_to_goal(
    &self,
    node: &StateSpaceNode,
  ) -> f64 {
    self.get_cost_to_adjacent_node(node, &self.goal_state_space_node)
  }

  fn get_adjacent_nodes(
    &self,
    node: &StateSpaceNode,
  ) -> Vec<StateSpaceNode> {
    let mut adjacent_list = Vec::<StateSpaceNode>::new();
    let distance_to_goal: f64 = node.distance(&self.goal_state_space_node);
    let distance_from_start: f64 = node.distance(&self.start_state_space_node);
    let mut step_size: f64 =
      (distance_from_start / self.init_step_size).trunc() * self.init_step_size;
    step_size = step_size.max(self.init_step_size);
    if distance_to_goal <= step_size {
      let adjacent_state_space_node = StateSpaceNode::new(
        self.goal_state_space_node.get_heading(),
        self.goal_state_space_node.get_point_xy(),
      );
      adjacent_list.push(adjacent_state_space_node);
      return adjacent_list;
    }
    let point_xy: Point2DD = node.get_point_xy();
    let x: f64 = point_xy.get_x();
    let y: f64 = point_xy.get_y();
    let goal_point_xy = self.goal_state_space_node.get_point_xy();
    // TODO: check if atan2 arguments reversed
    let heading_to_goal =
      (goal_point_xy.get_y() - y).atan2(goal_point_xy.get_x() - x);
    for i in 0..self.directions {
      let heading: f64 =
        heading_to_goal + (i as f64) * 2. * PI / (self.directions as f64);
      let adjacent_state_space_node: StateSpaceNode = StateSpaceNode::new(
        heading,
        Point2DD::new(
          x + step_size * heading.cos(),
          y + step_size * heading.sin(),
        ),
      );
      let point_xy = adjacent_state_space_node.get_point_xy();
      if self.is_space_available(point_xy.get_x(), point_xy.get_y())
        && self.push_adjacent_node(&adjacent_state_space_node)
      {
        adjacent_list.push(adjacent_state_space_node);
      }
    }
    adjacent_list
  }

  fn get_cost_to_adjacent_node(
    &self,
    from_node: &StateSpaceNode,
    to_node: &StateSpaceNode,
  ) -> f64 {
    let mut rotation: f64 = from_node.rotation(to_node);
    rotation = rotation.abs();
    let body_rotation_speed: f64 = self
      .tank
      .upgrade()
      .unwrap()
      .borrow()
      .get_body_rotation_speed();
    let rotation_time: f64 = rotation / body_rotation_speed;
    let travel_time: f64 = self.calculate_travel_time(from_node, to_node);
    let total_time: f64 = travel_time + rotation_time;
    total_time
  }

  fn is_goal_node(
    &self,
    node: &StateSpaceNode,
  ) -> bool {
    self.goal_state_space_node.distance(node) == 0.
  }
}
