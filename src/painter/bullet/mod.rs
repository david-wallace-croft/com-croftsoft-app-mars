// =============================================================================
//! - Bullet Painter for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023-2026 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-12
//! - Updated: 2026-09-03
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
use web_sys::CanvasRenderingContext2d;

pub struct BulletPainter {
  bullets: Rc<RefCell<VecDeque<Box<dyn Bullet>>>>,
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fill_style: &'static str,
  stroke_style: &'static str,
}

impl BulletPainter {
  pub fn new(
    bullets: Rc<RefCell<VecDeque<Box<dyn Bullet>>>>,
    context: Rc<RefCell<CanvasRenderingContext2d>>,
  ) -> Self {
    Self {
      bullets,
      context,
      fill_style: BULLET_FILL_STYLE,
      stroke_style: BULLET_STROKE_STYLE,
    }
  }
}

impl Painter for BulletPainter {
  fn paint(&self) {
    let context = self.context.borrow();
    context.set_fill_style_str(self.fill_style);
    context.set_stroke_style_str(self.stroke_style);
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
