// =============================================================================
//! - Default Game structure for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-07-03
//! - Updated: 2023-07-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::Game;
use core::cell::Cell;

#[derive(Default)]
pub struct DefaultGame {
  level: Cell<usize>,
}

impl Game for DefaultGame {
  fn get_level(&self) -> usize {
    self.level.get()
  }
}
