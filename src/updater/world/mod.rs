// =============================================================================
//! - World Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-30
//! - Updated: 2023-07-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::configuration::Configuration;
use crate::constant::{AMMO_DUMP_COUNT, OBSTACLE_COUNT};
use crate::game::Game;
use crate::options::Options;
use crate::updater::ammo_dump::AmmoDumpUpdater;
use crate::updater::bullet::BulletUpdater;
use crate::updater::explosion::ExplosionUpdater;
use crate::updater::obstacle::ObstacleUpdater;
use crate::updater::tank::TankUpdater;
use crate::updater::tank_operator::TankOperatorUpdater;
use crate::visitor::bullet::BulletVisitor;
use crate::visitor::explosion::ExplosionVisitor;
use crate::visitor::Visitor;
use crate::world::builder::WorldBuilder;
use crate::world::director::WorldBuilderDirector;
use crate::world::factory::WorldFactory;
use crate::world::seed::WorldSeed;
use crate::world::World;
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::rc::Rc;

pub trait WorldUpdaterInputs {
  fn get_reset_requested(&self) -> bool;
}

pub struct WorldUpdater {
  child_updaters: Vec<Box<dyn Updater>>,
  configuration: Configuration,
  factory: Rc<dyn WorldFactory>,
  game: Rc<dyn Game>,
  inputs: Rc<RefCell<dyn WorldUpdaterInputs>>,
  options: Rc<RefCell<Options>>,
  visitors: Vec<Box<dyn Visitor>>,
  world: Rc<dyn World>,
}

impl WorldUpdater {
  pub fn new(
    configuration: Configuration,
    factory: Rc<dyn WorldFactory>,
    game: Rc<dyn Game>,
    inputs: Rc<RefCell<dyn WorldUpdaterInputs>>,
    options: Rc<RefCell<Options>>,
    world: Rc<dyn World>,
  ) -> Self {
    let ammo_dump_updater = AmmoDumpUpdater::new(world.get_ammo_dumps());
    let bullet_updater = BulletUpdater::new(world.get_bullets());
    let explosion_updater = ExplosionUpdater::new(world.get_explosions());
    let obstacle_updater = ObstacleUpdater::new(world.get_obstacles());
    let tank_operator_updater = TankOperatorUpdater::new(world.clone());
    let tank_updater = TankUpdater::new(world.clone());
    let child_updaters: Vec<Box<dyn Updater>> = vec![
      Box::new(explosion_updater),
      Box::new(ammo_dump_updater),
      Box::new(tank_operator_updater),
      Box::new(tank_updater),
      Box::new(obstacle_updater),
      Box::new(bullet_updater),
    ];
    let bullet_visitor = BulletVisitor::new(world.clone());
    // TODO: add a collision detection visitor
    let explosion_visitor = ExplosionVisitor::new(world.clone());
    let visitors: Vec<Box<dyn Visitor>> = vec![
      Box::new(explosion_visitor),
      Box::new(bullet_visitor),
    ];
    Self {
      child_updaters,
      configuration,
      factory,
      game,
      inputs,
      options,
      visitors,
      world,
    }
  }

  fn reset(&self) {
    let world_builder = WorldBuilder {
      factory: self.factory.clone(),
      world: self.world.clone(),
    };
    let seed = WorldSeed {
      ammo_dump_count: AMMO_DUMP_COUNT,
      bounds: self.configuration.bounds,
      level: self.game.get_level(),
      obstacle_count: OBSTACLE_COUNT,
    };
    let world_builder_director = WorldBuilderDirector {
      seed,
      world_builder,
    };
    world_builder_director.direct();
  }
}

impl Updater for WorldUpdater {
  fn update(&mut self) {
    if self.inputs.borrow().get_reset_requested() {
      self.reset();
      return;
    }
    if self.options.borrow().pause {
      return;
    }
    self.child_updaters.iter_mut().for_each(|updater| updater.update());
    self
      .visitors
      .iter()
      .for_each(|visitor| self.world.accept_visitor(visitor.as_ref()));
  }
}
