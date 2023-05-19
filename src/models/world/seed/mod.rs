// =============================================================================
//! - World Seed for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-05-07
//! - Updated: 2023-05-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::builder::WorldBuilder;
use super::director::WorldBuilderDirector;
use super::factory::WorldFactory;
use super::World;
use com_croftsoft_core::math::geom::rectangle::Rectangle;
use core::cell::RefCell;
use std::rc::Rc;

pub struct WorldSeed {
  pub ammo_dump_count: usize,
  pub bounds: Rectangle,
  pub factory: Rc<RefCell<dyn WorldFactory>>,
  pub obstacle_count: usize,
}

impl WorldSeed {
  pub fn grow_world(&self) -> Rc<RefCell<World>> {
    let seed = WorldSeed {
      ammo_dump_count: self.ammo_dump_count,
      bounds: self.bounds,
      factory: self.factory.clone(),
      obstacle_count: self.obstacle_count,
    };
    let world = Rc::new(RefCell::new(World::new(self.factory.clone())));
    let world_builder = WorldBuilder::new(self.factory.clone(), world.clone());
    let world_director = WorldBuilderDirector {
      seed,
      world_builder,
    };
    world_director.direct();
    world
  }
}
