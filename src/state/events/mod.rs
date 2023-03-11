// =============================================================================
//! - Events for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-03-11
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

// use crate::updaters::root::RootUpdaterEvents;

#[derive(Default)]
pub struct Events {
  pub time_to_update: bool,
  pub updated: bool,
}

impl Events {
  pub fn clear(&mut self) {
    self.time_to_update = false;
    self.updated = false;
  }
}

// impl RootUpdaterEvents for Events {
//   fn get_time_to_update(&self) -> bool {
//     self.time_to_update
//   }

//   fn get_updated(&self) -> bool {
//     self.updated
//   }

//   fn set_time_to_update(&mut self) {
//     self.time_to_update = true;
//   }

//   fn set_updated(&mut self) {
//     self.updated = true;
//   }
// }
