// =============================================================================
//! - Ammo Dump Painter for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023-2026 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-04-30
//! - Updated: 2026-09-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constant::{AMMO_DUMP_FILL_STYLE, AMMO_DUMP_STROKE_STYLE};
use crate::model::ammo_dump::AmmoDump;
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_lib_role::Painter;
use core::cell::RefCell;
use core::f64::consts::TAU;
use std::collections::VecDeque;
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

pub struct AmmoDumpPainter {
  ammo_dumps: Rc<RefCell<VecDeque<Box<dyn AmmoDump>>>>,
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fill_style: &'static str,
  stroke_style: &'static str,
}

impl AmmoDumpPainter {
  pub fn new(
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    ammo_dumps: Rc<RefCell<VecDeque<Box<dyn AmmoDump>>>>,
  ) -> Self {
    Self {
      ammo_dumps,
      context,
      fill_style: AMMO_DUMP_FILL_STYLE,
      stroke_style: AMMO_DUMP_STROKE_STYLE,
    }
  }
}

impl Painter for AmmoDumpPainter {
  fn paint(&self) {
    let context = self.context.borrow();
    context.set_fill_style_str(self.fill_style);
    context.set_stroke_style_str(self.stroke_style);
    let ammo_dumps = self.ammo_dumps.borrow();
    ammo_dumps.iter().for_each(|ammo_dump| {
      let circle: Circle = ammo_dump.get_circle();
      context.begin_path();
      let _result =
        context.arc(circle.center_x, circle.center_y, circle.radius, 0., TAU);
      context.fill();
      context.stroke();
    });
  }
}
