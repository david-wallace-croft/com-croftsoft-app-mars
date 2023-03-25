// =============================================================================
//! - Overlay Painter for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-13
//! - Updated: 2023-03-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{FONT, OVERLAY_FILL_STYLE};
use crate::state::options::Options;
use crate::state::overlay::Overlay;
use com_croftsoft_lib_role::Painter;
use core::cell::{Ref, RefCell};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct OverlayPainter {
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fill_style: JsValue,
  options: Rc<RefCell<Options>>,
  overlay: Rc<RefCell<Overlay>>,
}

impl OverlayPainter {
  pub fn new(
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    options: Rc<RefCell<Options>>,
    overlay: Rc<RefCell<Overlay>>,
  ) -> Self {
    let fill_style: JsValue = JsValue::from_str(OVERLAY_FILL_STYLE);
    Self {
      context,
      fill_style,
      options,
      overlay,
    }
  }
}

impl Painter for OverlayPainter {
  fn paint(&mut self) {
    let options = self.options.borrow();
    if !options.update_rate_display || options.pause {
      return;
    }
    let context = self.context.borrow();
    context.set_fill_style(&self.fill_style);
    context.set_font(FONT);
    let overlay: Ref<Overlay> = self.overlay.borrow();
    context.fill_text(&overlay.update_rate_string, 4., 34.).unwrap();
  }
}
