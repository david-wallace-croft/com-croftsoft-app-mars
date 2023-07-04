// =============================================================================
//! - Options trait for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-07-03
//! - Updated: 2023-07-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub mod default;

pub trait Options {
  fn get_node_display(&self) -> bool;

  fn get_path_display(&self) -> bool;

  fn get_pause(&self) -> bool;

  fn get_update_rate_display(&self) -> bool;
}

pub trait OptionsMutator {
  fn set_node_display(
    &self,
    node_display: bool,
  );

  fn set_path_display(
    &self,
    path_display: bool,
  );

  fn set_pause(
    &self,
    pause: bool,
  );

  fn set_update_rate_display(
    &self,
    update_rate_display: bool,
  );
}
