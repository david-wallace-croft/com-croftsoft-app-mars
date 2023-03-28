// =============================================================================
//! - Obstacle Painter for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-15
//! - Updated: 2023-03-27
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::{OBSTACLE_FILL_STYLE, OBSTACLE_STROKE_STYLE};
use crate::models::obstacle::state::ObstacleState;
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_lib_role::Painter;
use core::cell::{Ref, RefCell};
use core::f64::consts::TAU;
use std::collections::VecDeque;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct ObstaclePainter {
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fill_style: JsValue,
  obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
  stroke_style: JsValue,
}

impl ObstaclePainter {
  pub fn new(
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
  ) -> Self {
    let fill_style: JsValue = JsValue::from_str(OBSTACLE_FILL_STYLE);
    let stroke_style: JsValue = JsValue::from_str(OBSTACLE_STROKE_STYLE);
    Self {
      context,
      fill_style,
      obstacles,
      stroke_style,
    }
  }
}

impl Painter for ObstaclePainter {
  fn paint(&mut self) {
    let context = self.context.borrow();
    context.set_fill_style(&self.fill_style);
    context.set_stroke_style(&self.stroke_style);
    let obstacles: Ref<VecDeque<ObstacleState>> = self.obstacles.borrow();
    for obstacle in obstacles.iter() {
      let Circle {
        center_x,
        center_y,
        radius,
      } = obstacle.circle;
      context.begin_path();
      let _result = context.arc(center_x, center_y, radius, 0., TAU);
      context.fill();
      context.stroke();
    }
  }
}
