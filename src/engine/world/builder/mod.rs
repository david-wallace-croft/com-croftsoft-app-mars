// =============================================================================
//! - World Builder for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-05-02
//! - Updated: 2023-06-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::factory::WorldFactory;
use super::World;
use crate::ai::tank_operator::TankOperator;
use crate::constants::{AMMO_DUMP_AMMO_MAX, OBSTACLE_RADIUS_MIN};
use crate::engine::traits::Color;
use crate::models::ammo_dump::default::DefaultAmmoDump;
use crate::models::obstacle::default::DefaultObstacle;
use crate::models::tank::default::DefaultTank;
use crate::models::tank::Tank;
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_core::math::geom::rectangle::Rectangle;
use core::cell::RefCell;
use std::rc::Rc;

pub struct WorldBuilder {
  factory: Rc<dyn WorldFactory>,
  pub world: Rc<dyn World>,
}

impl WorldBuilder {
  pub fn build_ammo_dump(
    &self,
    center_x: f64,
    center_y: f64,
    id: usize,
  ) {
    let ammo_dump = DefaultAmmoDump::new(
      AMMO_DUMP_AMMO_MAX,
      center_x,
      center_y,
      self.factory.clone(),
      id,
      self.world.clone(),
    );
    self.world.add_ammo_dump(Box::new(ammo_dump));
  }

  pub fn build_obstacle(
    &self,
    circle: Circle,
    drift_bounds: Rectangle,
    id: usize,
  ) {
    let obstacle = DefaultObstacle::new(
      circle,
      drift_bounds,
      id,
      OBSTACLE_RADIUS_MIN,
      self.world.clone(),
    );
    self.world.add_obstacle(Box::new(obstacle));
  }

  pub fn build_tank(
    &self,
    center_x: f64,
    center_y: f64,
    color: Color,
    id: usize,
  ) -> Rc<RefCell<dyn Tank>> {
    let tank_state = Rc::new(RefCell::new(DefaultTank::new(
      center_x,
      center_y,
      color,
      self.factory.clone(),
      id,
      self.world.clone(),
    )));
    // let tank_operator = Rc::new(RefCell::new(DefaultTankOperator::default()));
    // tank_state.borrow_mut().set_tank_operator(tank_operator.clone());
    // tank_operator.borrow_mut().set_tank_console(tank_state.clone());
    // TODO: model_array_keep.insert(seriTank) was in the old code here
    self.world.add_tank(tank_state.clone());
    tank_state
  }

  pub fn build_tank_operator(
    &self,
    tank: Rc<RefCell<dyn Tank>>,
  ) {
    let tank_operator: Rc<RefCell<dyn TankOperator>> =
      self.factory.make_tank_operator(tank);
    self.world.add_tank_operator(tank_operator);
  }

  pub fn new(factory: Rc<dyn WorldFactory>) -> Self {
    let world = factory.make_world();
    Self {
      factory,
      world,
    }
  }
}
