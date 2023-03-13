#![allow(clippy::uninlined_format_args)]

use com_croftsoft_lib_animation::web_sys::log;
use constants::INFO;
use engine::looper::Looper;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

pub mod components;
pub mod constants;
pub mod engine;
pub mod painters;
pub mod state;
pub mod updaters;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();
  log(INFO);
  Looper::launch();
  Ok(())
}
