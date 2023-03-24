// =============================================================================
//! - Obstacle Painter for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-15
//! - Updated: 2023-03-23
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constants::FILL_STYLE_OBSTACLE;
use crate::state::obstacle::ObstacleState;
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
}

impl ObstaclePainter {
  pub fn new(
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    obstacles: Rc<RefCell<VecDeque<ObstacleState>>>,
  ) -> Self {
    let fill_style: JsValue = JsValue::from_str(FILL_STYLE_OBSTACLE);
    Self {
      context,
      fill_style,
      obstacles,
    }
  }
}

impl Painter for ObstaclePainter {
  fn paint(&mut self) {
    let context = self.context.borrow();
    context.set_fill_style(&self.fill_style);
    let obstacles: Ref<VecDeque<ObstacleState>> = self.obstacles.borrow();
    for obstacle in obstacles.iter() {
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
}
