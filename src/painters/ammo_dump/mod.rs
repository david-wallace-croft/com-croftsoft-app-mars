// =============================================================================
//! - Ammo Dump Painter for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-30
//! - Updated: 2023-04-30
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{AMMO_DUMP_FILL_STYLE, AMMO_DUMP_STROKE_STYLE};
use crate::engine::traits::ModelAccessor;
use crate::models::ammo_dump::default::DefaultAmmoDump;
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_lib_role::Painter;
use core::cell::RefCell;
use core::f64::consts::TAU;
use std::collections::VecDeque;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct AmmoDumpPainter {
  // TODO: maybe dyn AmmoDump instead of DefaultAmmoDump
  // TODO: can this be a Vec instead of a VecDeque?
  ammo_dumps: Rc<RefCell<VecDeque<DefaultAmmoDump>>>,
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fill_style: JsValue,
  stroke_style: JsValue,
}

impl AmmoDumpPainter {
  pub fn new(
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    ammo_dumps: Rc<RefCell<VecDeque<DefaultAmmoDump>>>,
  ) -> Self {
    let fill_style: JsValue = JsValue::from_str(AMMO_DUMP_FILL_STYLE);
    let stroke_style: JsValue = JsValue::from_str(AMMO_DUMP_STROKE_STYLE);
    Self {
      ammo_dumps,
      context,
      fill_style,
      stroke_style,
    }
  }
}

impl Painter for AmmoDumpPainter {
  fn paint(&mut self) {
    let context = self.context.borrow();
    context.set_fill_style(&self.fill_style);
    context.set_stroke_style(&self.stroke_style);
    let ammo_dumps = self.ammo_dumps.borrow();
    let mut circle = Circle::default();
    ammo_dumps.iter().for_each(|ammo_dump| {
      circle = ammo_dump.get_shape(circle);
      context.begin_path();
      let _result =
        context.arc(circle.center_x, circle.center_y, circle.radius, 0., TAU);
      context.fill();
      context.stroke();
    });
  }
}
