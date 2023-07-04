// =============================================================================
//! - Default Options structure for CroftSoft Mars
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

use super::Options;
use core::cell::Cell;

#[derive(Default)]
pub struct DefaultOptions {
  node_display: Cell<bool>,
  path_display: Cell<bool>,
  pause: Cell<bool>,
  update_rate_display: Cell<bool>,
}

impl Options for DefaultOptions {
  fn get_node_display(&self) -> bool {
    self.node_display.get()
  }

  fn get_path_display(&self) -> bool {
    self.path_display.get()
  }

  fn get_pause(&self) -> bool {
    self.pause.get()
  }

  fn get_update_rate_display(&self) -> bool {
    self.update_rate_display.get()
  }

  fn set_node_display(
    &self,
    node_display: bool,
  ) {
    self.node_display.set(node_display);
  }

  fn set_path_display(
    &self,
    path_display: bool,
  ) {
    self.path_display.set(path_display);
  }

  fn set_pause(
    &self,
    pause: bool,
  ) {
    self.pause.set(pause);
  }

  fn set_update_rate_display(
    &self,
    update_rate_display: bool,
  ) {
    self.update_rate_display.set(update_rate_display);
  }
}
