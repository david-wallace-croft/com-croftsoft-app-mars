// =============================================================================
//! - Root Updater Options for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-03
//! - Updated: 2023-06-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::options::Options;
use crate::updater::overlay::OverlayUpdaterOptions;
use core::cell::RefCell;
use std::rc::Rc;

impl RootUpdaterOptions for Options {
  fn get_pause(&self) -> bool {
    self.pause
  }

  fn get_update_rate_display(&self) -> bool {
    self.update_rate_display
  }
}

pub trait RootUpdaterOptions {
  fn get_pause(&self) -> bool;
  // fn get_time_display(&self) -> bool;
  fn get_update_rate_display(&self) -> bool;
}

pub struct RootUpdaterOptionsAdapter {
  options: Rc<RefCell<dyn RootUpdaterOptions>>,
}

impl RootUpdaterOptionsAdapter {
  pub fn new(options: Rc<RefCell<dyn RootUpdaterOptions>>) -> Self {
    Self {
      options,
    }
  }
}

impl OverlayUpdaterOptions for RootUpdaterOptionsAdapter {
  fn get_pause(&self) -> bool {
    self.options.borrow().get_pause()
  }

  fn get_update_rate_display(&self) -> bool {
    self.options.borrow().get_update_rate_display()
  }
}
