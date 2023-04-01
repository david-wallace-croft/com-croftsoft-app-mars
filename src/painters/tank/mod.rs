// =============================================================================
//! - Tank Painter for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-31
//! - Updated: 2023-04-01
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{TANK_FILL_STYLE, TANK_STROKE_STYLE};
use crate::engine::traits::ModelAccessor;
use crate::models::tank::state::TankState;
use crate::models::tank::TankAccessor;
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_lib_role::Painter;
use core::cell::{Ref, RefCell};
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
    let fill_style: JsValue = JsValue::from_str(TANK_FILL_STYLE);
    let stroke_style: JsValue = JsValue::from_str(TANK_STROKE_STYLE);
    Self {
      context,
      fill_style,
      stroke_style,
      tanks,
    }
  }

  fn paint_tank_body(
    &self,
    tank: &TankState,
  ) {
    let mut circle: Circle = Circle::default();
    circle = tank.get_shape(circle);
    let center_x: f64 = circle.center_x;
    let center_y: f64 = circle.center_y;
    let context = self.context.borrow();
    context.save();
    let _result = context.translate(center_x, center_y);
    let _result = context.rotate(tank.get_body_heading());
    // let _result = context.translate(-center_x, -center_y);
    // TODO: rescale this in terms of TANK_RADIUS
    let x: f64 = -25.;
    let y: f64 = -15.;
    let w: f64 = 50.;
    let h: f64 = 30.;
    context.set_fill_style(&self.fill_style);
    context.set_stroke_style(&self.stroke_style);
    context.begin_path();
    context.rect(x, y, w, h);
    context.fill();
    context.stroke();
    let x: f64 = 21.;
    let y: f64 = -2.;
    let w: f64 = 2.;
    let h: f64 = 4.;
    context.set_fill_style(&self.stroke_style);
    context.begin_path();
    context.rect(x, y, w, h);
    context.fill();
    context.restore();
    // let _result = context.translate(0., 0.);
    // let _result = context.rotate(-PI / 4.);
  }
}

impl Painter for TankPainter {
  fn paint(&mut self) {
    let context = self.context.borrow();
    context.set_fill_style(&self.fill_style);
    context.set_stroke_style(&self.stroke_style);
    let tanks: Ref<VecDeque<TankState>> = self.tanks.borrow();
    for tank in tanks.iter() {
      self.paint_tank_body(tank);
    }
  }
}
