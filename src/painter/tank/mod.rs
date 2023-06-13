// =============================================================================
//! - Tank Painter for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-31
//! - Updated: 2023-06-12
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::constant::{
  TANK_FILL_STYLE_BLUE, TANK_FILL_STYLE_RED, TANK_FILL_STYLE_SPARKING,
  TANK_STROKE_STYLE,
};
use crate::model::tank::{Color, Tank};
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
  fill_style_blue: JsValue,
  fill_style_red: JsValue,
  fill_style_sparking: JsValue,
  stroke_style: JsValue,
  tanks: Rc<RefCell<VecDeque<Rc<RefCell<dyn Tank>>>>>,
}

impl TankPainter {
  pub fn new(
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<dyn Tank>>>>>,
  ) -> Self {
    let fill_style_blue: JsValue = JsValue::from_str(TANK_FILL_STYLE_BLUE);
    let fill_style_red: JsValue = JsValue::from_str(TANK_FILL_STYLE_RED);
    let fill_style_sparking: JsValue =
      JsValue::from_str(TANK_FILL_STYLE_SPARKING);
    let stroke_style: JsValue = JsValue::from_str(TANK_STROKE_STYLE);
    Self {
      context,
      fill_style_blue,
      fill_style_red,
      fill_style_sparking,
      stroke_style,
      tanks,
    }
  }

  fn paint_tank(
    &self,
    // TODO: Can this take TankAccessor
    tank: &dyn Tank,
  ) -> Result<(), JsValue> {
    let circle: Circle = tank.get_circle();
    let center_x: f64 = circle.center_x;
    let center_y: f64 = circle.center_y;
    let context = self.context.borrow();
    context.save();
    let _result = context.translate(center_x, center_y);
    let _result = context.rotate(tank.get_body_heading());
    let fill_style = match tank.get_color() {
      Color::RED => &self.fill_style_red,
      Color::BLUE => &self.fill_style_blue,
    };
    context.set_fill_style(fill_style);
    context.set_stroke_style(&self.stroke_style);
    // TODO: rescale this in terms of TANK_RADIUS
    // tank treads
    let x: f64 = -25.;
    let y: f64 = -25.;
    let w: f64 = 50.;
    let h: f64 = 10.;
    context.begin_path();
    context.rect(x, y, w, h);
    context.fill();
    context.stroke();
    let x: f64 = -25.;
    let y: f64 = 15.;
    let w: f64 = 50.;
    let h: f64 = 10.;
    context.begin_path();
    context.rect(x, y, w, h);
    context.fill();
    context.stroke();
    context.begin_path();
    for index in -5..=4 {
      let x: f64 = (index * 5) as f64;
      context.move_to(x + tank.get_tread_offset_left(), -25.);
      context.line_to(x + tank.get_tread_offset_left(), -15.);
      context.move_to(x + tank.get_tread_offset_right(), 15.);
      context.line_to(x + tank.get_tread_offset_right(), 25.);
    }
    context.stroke();
    // tank body
    let x: f64 = -25.;
    let y: f64 = -15.;
    let w: f64 = 50.;
    let h: f64 = 30.;
    context.begin_path();
    context.rect(x, y, w, h);
    context.fill();
    context.stroke();
    // tank body window
    let x: f64 = 21.;
    let y: f64 = -2.;
    let w: f64 = 2.;
    let h: f64 = 4.;
    context.set_fill_style(&self.stroke_style);
    context.begin_path();
    context.rect(x, y, w, h);
    context.fill();
    context.restore();
    context.save();
    context.translate(center_x, center_y)?;
    context.rotate(tank.get_turret_heading())?;
    // tank turret
    context.begin_path();
    context.arc(0., 0., 10., 0., TAU)?;
    context.stroke();
    // tank turret hatch
    context.begin_path();
    context.arc(0., 0., 4., 0., TAU)?;
    context.stroke();
    // tank turret cannon mount
    context.begin_path();
    context.rect(10., -2., 4., 4.);
    context.stroke();
    context.set_fill_style(fill_style);
    // tank cannon
    context.begin_path();
    context.rect(14., -1., 11., 2.);
    context.fill();
    context.stroke();
    // sparking
    if tank.is_burning() || tank.is_sparking() {
      context.set_fill_style(&self.fill_style_sparking);
      context.begin_path();
      context.rect(-5., -5., 10., 10.);
      context.fill();
    }
    // end
    context.restore();
    Ok(())
  }
}

impl Painter for TankPainter {
  fn paint(&mut self) {
    let tanks: Ref<VecDeque<Rc<RefCell<dyn Tank>>>> = self.tanks.borrow();
    for tank in tanks.iter() {
      let tank: Ref<dyn Tank> = tank.borrow();
      if !tank.is_active() {
        // TODO: remove inactive tanks
        continue;
      }
      let _result = self.paint_tank(&*tank);
    }
  }
}
