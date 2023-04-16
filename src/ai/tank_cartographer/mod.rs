// =============================================================================
//! - Tank Cartographer for CroftSoft Mars
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

use super::state_space_node::StateSpaceNode;
use crate::ai::tank_console::TankConsole;
use com_croftsoft_core::ai::astar::traits::Cartographer;
use com_croftsoft_core::math::geom::point_2dd::Point2DD;
use com_croftsoft_core::math::geom::point_xy::PointXY;
use std::cell::RefCell;
use std::f64::consts::PI;
use std::f64::INFINITY;
use std::rc::Rc;

pub struct TankCartographer {
  directions: usize,
  goal_state_space_node: StateSpaceNode,
  init_step_size: f64,
  start_state_space_node: StateSpaceNode,
  tank_console: Option<Rc<RefCell<dyn TankConsole>>>,
}

impl TankCartographer {
  fn calculate_travel_time(
    &self,
    from_node: &StateSpaceNode,
    to_node: &StateSpaceNode,
  ) -> f64 {
    let distance: f64 = from_node.distance(to_node);
    if let Some(tank_console) = &self.tank_console {
      distance / tank_console.borrow().get_tank_speed()
    } else {
      // TODO: what if tank_console is None?
      INFINITY
    }
  }

  pub fn new(
    init_step_size: f64,
    directions: usize,
  ) -> Self {
    let goal_state_space_node = StateSpaceNode::new(0., Point2DD::default());
    let start_state_space_node = StateSpaceNode::new(0., Point2DD::default());
    let tank_console = None;
    TankCartographer {
      directions,
      goal_state_space_node,
      init_step_size,
      start_state_space_node,
      tank_console,
    }
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

  pub fn set_tank_console(
    &mut self,
    tank_console: Rc<RefCell<dyn TankConsole>>,
  ) {
    self.tank_console = Some(tank_console);
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
      adjacent_list.push(StateSpaceNode::new(
        self.goal_state_space_node.get_heading(),
        self.goal_state_space_node.get_point_xy(),
      ));
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
      if let Some(tank_console) = &self.tank_console {
        if tank_console.borrow().is_space_available(
          adjacent_state_space_node.get_point_xy().get_x(),
          adjacent_state_space_node.get_point_xy().get_y(),
        ) {
          adjacent_list.push(adjacent_state_space_node);
        }
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
    if let Some(tank_console) = &self.tank_console {
      let body_rotation_speed: f64 =
        tank_console.borrow().get_body_rotation_speed();
      let rotation_time: f64 = rotation / body_rotation_speed;
      let travel_time: f64 = self.calculate_travel_time(from_node, to_node);
      let total_time: f64 = travel_time + rotation_time;
      total_time
    } else {
      // TODO: what if tank_console is None?
      INFINITY
    }
  }

  fn is_goal_node(
    &self,
    node: &StateSpaceNode,
  ) -> bool {
    self.goal_state_space_node.distance(node) == 0.
  }
}