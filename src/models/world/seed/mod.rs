// =============================================================================
//! - World Seed for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-05-07
//! - Updated: 2023-05-20
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::builder::WorldBuilder;
use super::default::DefaultWorld;
use super::director::WorldBuilderDirector;
use super::factory::WorldFactory;
use super::World;
use com_croftsoft_core::math::geom::rectangle::Rectangle;
use core::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy)]
pub struct WorldSeed {
  pub ammo_dump_count: usize,
  pub bounds: Rectangle,
  pub obstacle_count: usize,
}

impl WorldSeed {
  pub fn grow_world(
    &self,
    factory: Rc<dyn WorldFactory>,
  ) -> Rc<RefCell<dyn World>> {
    // TODO: use WorldFactory to make the World
    let world = Rc::new(RefCell::new(DefaultWorld::new(factory)));
    let world_builder = WorldBuilder::new(world.clone());
    let world_director = WorldBuilderDirector {
      seed: *self,
      world_builder,
    };
    world_director.direct();
    world
  }
}
