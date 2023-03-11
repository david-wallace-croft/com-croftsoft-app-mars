#![allow(clippy::uninlined_format_args)]

use com_croftsoft_lib_animation::web_sys::log;
use constants::INFO;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

pub mod constants;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();
  log(INFO);
  Ok(())
}
