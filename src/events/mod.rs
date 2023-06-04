// =============================================================================
//! - Events for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-06-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

// TODO: maybe refactor into individual messages in a queue

#[derive(Default)]
pub struct Events {
  pub time_to_update: bool,
  pub updated: bool,
  pub update_period_millis_changed: Option<f64>,
}

impl Events {
  pub fn clear(&mut self) {
    self.time_to_update = false;
    self.update_period_millis_changed = None;
    self.updated = false;
  }
}
