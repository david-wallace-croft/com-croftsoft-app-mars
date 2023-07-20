// =============================================================================
//! - Main function for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-07-20
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_lib_animation::web_sys::log;
use constant::INFO;
use looper::Looper;
use wasm_bindgen::prelude::*;

pub mod ai;
pub mod component;
pub mod configuration;
pub mod constant;
pub mod events;
pub mod game;
pub mod inputs;
pub mod looper;
pub mod model;
pub mod options;
pub mod overlay;
pub mod painter;
pub mod preparer;
pub mod root;
pub mod updater;
pub mod visitor;
pub mod world;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();
  log(INFO);
  Looper::launch();
  Ok(())
}
