// =============================================================================
//! - Obstacle Painter for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-15
//! - Updated: 2023-03-15
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::FILL_STYLE_OBSTACLE;
use crate::state::obstacle::Obstacle;
use com_croftsoft_core::math::geom::structures::Circle;
use com_croftsoft_lib_role::Painter;
use core::cell::{Ref, RefCell};
use core::f64::consts::TAU;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct ObstaclePainter {
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fill_style: JsValue,
  obstacle: Rc<RefCell<Obstacle>>,
}

impl ObstaclePainter {
  pub fn new(
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    obstacle: Rc<RefCell<Obstacle>>,
  ) -> Self {
    let fill_style: JsValue = JsValue::from_str(FILL_STYLE_OBSTACLE);
    Self {
      context,
      fill_style,
      obstacle,
    }
  }
}

impl Painter for ObstaclePainter {
  fn paint(&mut self) {
    let context = self.context.borrow();
    context.set_fill_style(&self.fill_style);
    let obstacle: Ref<Obstacle> = self.obstacle.borrow();
    let Circle {
      center_x,
      center_y,
      radius,
    } = obstacle.circle;
    context.begin_path();
    let _result = context.arc(center_x, center_y, radius, 0., TAU);
    context.stroke();
  }
}
