// =============================================================================
//! - Visitor for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-04
//! - Updated: 2023-06-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::model::ammo_dump::AmmoDump;
use crate::model::obstacle::Obstacle;
use crate::model::tank::Tank;

pub mod bullet;

pub trait Visitor {
  fn visit_ammo_dump(
    &self,
    ammo_dump: &mut dyn AmmoDump,
  );

  fn visit_obstacle(
    &self,
    obstacle: &mut dyn Obstacle,
  );

  fn visit_tank(
    &self,
    tank: &mut dyn Tank,
  );
}

pub trait VisitorAcceptor {
  fn accept_visitor(
    &self,
    visitor: &dyn Visitor,
  );
}
