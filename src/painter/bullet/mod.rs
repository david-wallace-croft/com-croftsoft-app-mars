// =============================================================================
//! - Bullet Painter for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-12
//! - Updated: 2023-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constant::{BULLET_FILL_STYLE, BULLET_STROKE_STYLE};
use crate::model::bullet::Bullet;
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_lib_role::Painter;
use core::cell::RefCell;
use core::f64::consts::TAU;
use std::collections::VecDeque;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct BulletPainter {
  bullets: Rc<RefCell<VecDeque<Box<dyn Bullet>>>>,
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fill_style: JsValue,
  stroke_style: JsValue,
}

impl BulletPainter {
  pub fn new(
    bullets: Rc<RefCell<VecDeque<Box<dyn Bullet>>>>,
    context: Rc<RefCell<CanvasRenderingContext2d>>,
  ) -> Self {
    let fill_style: JsValue = JsValue::from_str(BULLET_FILL_STYLE);
    let stroke_style: JsValue = JsValue::from_str(BULLET_STROKE_STYLE);
    Self {
      bullets,
      context,
      fill_style,
      stroke_style,
    }
  }
}

impl Painter for BulletPainter {
  fn paint(&self) {
    let context = self.context.borrow();
    context.set_fill_style(&self.fill_style);
    context.set_stroke_style(&self.stroke_style);
    let bullets = self.bullets.borrow();
    bullets.iter().for_each(|bullet| {
      let circle: Circle = bullet.get_circle();
      context.begin_path();
      let _result =
        context.arc(circle.center_x, circle.center_y, circle.radius, 0., TAU);
      context.fill();
      context.stroke();
    });
  }
}
