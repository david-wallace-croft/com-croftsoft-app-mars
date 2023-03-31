// =============================================================================
//! - Tank Painter for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-31
//! - Updated: 2023-03-31
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{OBSTACLE_FILL_STYLE, OBSTACLE_STROKE_STYLE};
use crate::engine::traits::ModelAccessor;
use crate::models::tank::state::TankState;
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_lib_role::Painter;
use core::cell::{Ref, RefCell};
use core::f64::consts::TAU;
use std::collections::VecDeque;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct TankPainter {
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fill_style: JsValue,
  stroke_style: JsValue,
  // TODO: change this to dyn TankAccessor
  tanks: Rc<RefCell<VecDeque<TankState>>>,
}

impl TankPainter {
  pub fn new(
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    tanks: Rc<RefCell<VecDeque<TankState>>>,
  ) -> Self {
    let fill_style: JsValue = JsValue::from_str(OBSTACLE_FILL_STYLE);
    let stroke_style: JsValue = JsValue::from_str(OBSTACLE_STROKE_STYLE);
    Self {
      context,
      fill_style,
      stroke_style,
      tanks,
    }
  }
}

impl Painter for TankPainter {
  fn paint(&mut self) {
    let context = self.context.borrow();
    context.set_fill_style(&self.fill_style);
    context.set_stroke_style(&self.stroke_style);
    let tanks: Ref<VecDeque<TankState>> = self.tanks.borrow();
    let mut circle = Circle::default();
    for tank in tanks.iter() {
      circle = tank.get_shape(circle);
      context.begin_path();
      let _result =
        context.arc(circle.center_x, circle.center_y, circle.radius, 0., TAU);
      context.fill();
      context.stroke();
    }
  }
}
