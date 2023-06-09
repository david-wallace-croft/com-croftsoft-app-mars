// =============================================================================
//! - World Seed for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-07
//! - Updated: 2023-07-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_core::math::geom::rectangle::Rectangle;

pub struct WorldSeed {
  pub bounds: Rectangle,
  pub level: usize,
}
