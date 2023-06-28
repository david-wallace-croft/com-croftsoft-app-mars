// =============================================================================
//! - World Builder for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-02
//! - Updated: 2023-06-28
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::factory::WorldFactory;
use super::World;
use crate::ai::tank_operator::default::DefaultTankOperator;
use crate::constant::{AMMO_DUMP_AMMO_MAX, OBSTACLE_RADIUS_MIN};
use crate::model::ammo_dump::default::DefaultAmmoDump;
use crate::model::obstacle::default::DefaultObstacle;
use crate::model::tank::default::DefaultTank;
use crate::model::tank::{Color, Tank};
use crate::model::ModelAccessor;
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_core::math::geom::rectangle::Rectangle;
use core::cell::RefCell;
use std::rc::Rc;

pub struct WorldBuilder {
  pub factory: Rc<dyn WorldFactory>,
  pub world: Rc<dyn World>,
}

impl WorldBuilder {
  pub fn build_ammo_dump(
    &self,
    center_x: f64,
    center_y: f64,
    id: usize,
  ) {
    // TODO: Use Factory to instantiate these
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

  pub fn build_tank_operator(
    &self,
    body_heading: f64,
    center_x: f64,
    center_y: f64,
    color: Color,
    id: usize,
    turret_heading: f64,
  ) {
    let tank: Rc<RefCell<DefaultTank>> =
      Rc::new(RefCell::new(DefaultTank::new(
        center_x,
        center_y,
        color,
        self.factory.clone(),
        id,
        self.world.clone(),
      )));
    tank.borrow_mut().set_body_heading(body_heading);
    tank.borrow_mut().set_turret_heading(turret_heading);
    let tank_operator = DefaultTankOperator::new(
      tank.borrow().get_id(),
      tank.clone(),
      self.world.clone(),
    );
    self.world.add_tank_operator(Box::new(tank_operator));
  }
}
