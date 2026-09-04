// =============================================================================
//! - Explosion Painter for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023-2026 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-05-21
//! - Updated: 2026-09-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constant::{EXPLOSION_FILL_STYLE, EXPLOSION_STROKE_STYLE};
use crate::model::explosion::Explosion;
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_lib_role::Painter;
use core::cell::{Ref, RefCell};
use core::f64::consts::TAU;
use std::collections::VecDeque;
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

pub struct ExplosionPainter {
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  explosions: Rc<RefCell<VecDeque<Box<dyn Explosion>>>>,
  fill_style: &'static str,
  stroke_style: &'static str,
}

impl ExplosionPainter {
  pub fn new(
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    explosions: Rc<RefCell<VecDeque<Box<dyn Explosion>>>>,
  ) -> Self {
    Self {
      context,
      explosions,
      fill_style: EXPLOSION_FILL_STYLE,
      stroke_style: EXPLOSION_STROKE_STYLE,
    }
  }
}

impl Painter for ExplosionPainter {
  fn paint(&self) {
    let context = self.context.borrow();
    context.set_fill_style_str(self.fill_style);
    context.set_stroke_style_str(self.stroke_style);
    let explosions: Ref<VecDeque<Box<dyn Explosion>>> =
      self.explosions.borrow();
    for explosion in explosions.iter() {
      let circle: Circle = explosion.get_circle();
      context.begin_path();
      let _result =
        context.arc(circle.center_x, circle.center_y, circle.radius, 0., TAU);
      context.fill();
      context.stroke();
    }
  }
}
