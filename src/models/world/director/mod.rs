// =============================================================================
//! - World Director for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-05-03
//! - Updated: 2023-05-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::builder::WorldBuilder;
use super::World;
use crate::ai::tank_console::default::DefaultTankConsole;
use crate::constants::{
  OBSTACLE_COUNT, OBSTACLE_RADIUS_MAX, OBSTACLE_RADIUS_MIN,
  OBSTACLE_RANDOM_PLACEMENT_ATTEMPTS_MAX,
};
use crate::engine::traits::Color;
use crate::models::tank::state::TankState;
use crate::models::tank::TankMutator;
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_core::math::geom::rectangle::Rectangle;
use core::cell::RefCell;
use core::f64::consts::TAU;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use std::rc::Rc;

pub struct WorldDirector {
  bounds: Rectangle,
  world_builder: WorldBuilder,
}

impl WorldDirector {
  fn make_ammo_dumps(&self) {
    for index in 0..5 {
      let offset = ((index + 1) * 100) as f64;
      self.world_builder.build_ammo_dump(offset, offset, index);
    }
  }

  pub fn make_level(&self) -> Rc<RefCell<World>> {
    self.make_ammo_dumps();
    self.make_tanks();
    self.make_obstacles();
    self.make_tank_consoles();
    self.world_builder.world.clone()
  }

  fn make_obstacles(&self) {
    let mut rng = rand::thread_rng();
    let center_uniform = Uniform::from(self.bounds.x_min..=self.bounds.x_max);
    let radius_uniform =
      Uniform::from(OBSTACLE_RADIUS_MIN..=OBSTACLE_RADIUS_MAX);
    for _ in 0..OBSTACLE_COUNT {
      let center_x = center_uniform.sample(&mut rng);
      let center_y = center_uniform.sample(&mut rng);
      let radius = radius_uniform.sample(&mut rng);
      let circle = Circle {
        center_x,
        center_y,
        radius,
      };
      self.world_builder.build_obstacle(circle, self.bounds);
    }
    for mut obstacle in
      self.world_builder.world.borrow().obstacles.borrow_mut().iter_mut()
    {
      for _ in 0..OBSTACLE_RANDOM_PLACEMENT_ATTEMPTS_MAX {
        // TODO: Also check to see if blocked by something else
        if self
          .world_builder
          .world
          .borrow()
          .is_blocked_by_tank(&obstacle.circle)
        {
          break;
        }
        obstacle.circle.center_x = center_uniform.sample(&mut rng);
        obstacle.circle.center_y = center_uniform.sample(&mut rng);
      }
    }
  }

  fn make_tank_consoles(&self) {
    let world = self.world_builder.world.borrow();
    let tanks = world.tanks.borrow();
    let length = tanks.len();
    for index in 0..length {
      let tank = tanks[index].clone();
      let tank_console = Rc::new(RefCell::new(DefaultTankConsole {
        tank,
        world: self.world_builder.world.clone(),
      }));
      self.world_builder.world.borrow().tank_operators.borrow()[index]
        .borrow_mut()
        .set_tank_console(tank_console);
    }
  }

  fn make_tanks(&self) {
    for index in 0..6 {
      let center_x: f64 = if index >= 3 {
        (index * 200 - 500) as f64
      } else {
        (index * 200 + 100) as f64
      };
      let center_y: f64 = if index >= 3 {
        100.
      } else {
        500.
      };
      let color = if index >= 3 {
        Color::RED
      } else {
        Color::BLUE
      };
      let tank: Rc<RefCell<TankState>> =
        self.world_builder.build_tank(center_x, center_y, color, index);
      tank.borrow_mut().set_body_heading(((index) as f64) * TAU / 8.);
      tank.borrow_mut().set_turret_heading(((index) as f64) * TAU / 4.);
      self.world_builder.build_tank_operator(index);
    }
  }

  pub fn new(bounds: Rectangle) -> Self {
    let world_builder = WorldBuilder::default();
    Self {
      bounds,
      world_builder,
    }
  }

  // pub fn update(&self) {
  //   // TODO: copy in the rest of the old code
  //   self.make_level();
  // }
}
