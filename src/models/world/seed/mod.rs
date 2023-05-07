// =============================================================================
//! - World Seed for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-05-07
//! - Updated: 2023-05-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::builder::WorldBuilder;
use super::director::WorldBuilderDirector;
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
  pub fn grow_world(&self) -> Rc<RefCell<World>> {
    let seed: WorldSeed = *self;
    let world_builder = WorldBuilder::default();
    let world_director = WorldBuilderDirector {
      seed,
      world_builder,
    };
    world_director.direct();
    world_director.world_builder.world
  }
}
