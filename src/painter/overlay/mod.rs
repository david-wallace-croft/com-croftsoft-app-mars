// =============================================================================
//! - Overlay Painter for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023-2026 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-13
//! - Updated: 2026-09-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constant::{FONT, OVERLAY_FILL_STYLE};
use crate::options::Options;
use crate::overlay::Overlay;
use com_croftsoft_lib_role::Painter;
use core::cell::{Ref, RefCell};
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

pub struct OverlayPainter {
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fill_style: &'static str,
  options: Rc<dyn Options>,
  overlay: Rc<RefCell<Overlay>>,
}

impl OverlayPainter {
  pub fn new(
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    options: Rc<dyn Options>,
    overlay: Rc<RefCell<Overlay>>,
  ) -> Self {
    Self {
      context,
      fill_style: OVERLAY_FILL_STYLE,
      options,
      overlay,
    }
  }
}

impl Painter for OverlayPainter {
  fn paint(&self) {
    if self.options.get_pause() || !self.options.get_update_rate_display() {
      return;
    }
    let context = self.context.borrow();
    context.set_fill_style_str(self.fill_style);
    context.set_font(FONT);
    let overlay: Ref<Overlay> = self.overlay.borrow();
    context
      .fill_text(&overlay.update_rate_string, 4., 34.)
      .unwrap();
  }
}
