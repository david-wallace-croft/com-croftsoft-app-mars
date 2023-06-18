// =============================================================================
//! - Path Painter for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-18
//! - Updated: 2023-06-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::ai::state_space_node::StateSpaceNode;
use crate::constant::{
  TANK_FILL_STYLE_BLUE, TANK_FILL_STYLE_RED,
  TANK_RADIUS,
};
use crate::model::tank::{Color, Tank};
use com_croftsoft_lib_role::Painter;
use core::cell::{Ref, RefCell};
use core::f64::consts::TAU;
use std::collections::VecDeque;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct PathPainter {
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fill_style_blue: JsValue,
  fill_style_red: JsValue,
  tanks: Rc<RefCell<VecDeque<Rc<RefCell<dyn Tank>>>>>,
}

impl PathPainter {
  pub fn new(
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<dyn Tank>>>>>,
  ) -> Self {
    let fill_style_blue: JsValue = JsValue::from_str(TANK_FILL_STYLE_BLUE);
    let fill_style_red: JsValue = JsValue::from_str(TANK_FILL_STYLE_RED);
    Self {
      context,
      fill_style_blue,
      fill_style_red,
      tanks,
    }
  }

  fn paint_path(
    &self,
    // TODO: Can this take TankAccessor
    tank: &dyn Tank,
  ) -> Result<(), JsValue> {
    let context = self.context.borrow();
    context.save();
    let stroke_style = match tank.get_color() {
      Color::RED => &self.fill_style_red,
      Color::BLUE => &self.fill_style_blue,
    };
    context.set_stroke_style(stroke_style);
    context.set_line_width(3.);
    if let Some(tank_operator) = tank.get_tank_operator() {
      let tank_operator = tank_operator.borrow();
      let state_space_nodes: Vec<StateSpaceNode> = tank_operator.get_path();
      state_space_nodes.iter().for_each(|state_space_node| {
        let point_2dd = state_space_node.get_point_xy();
        // TODO: show state space node heading
        context.begin_path();
        let _result = context.arc(point_2dd.x, point_2dd.y, TANK_RADIUS, 0., TAU);
        context.stroke();
      });
    }
    context.restore();
    Ok(())
  }
}

impl Painter for PathPainter {
  fn paint(&mut self) {
    let tanks: Ref<VecDeque<Rc<RefCell<dyn Tank>>>> = self.tanks.borrow();
    for tank in tanks.iter() {
      let tank: Ref<dyn Tank> = tank.borrow();
      let _result = self.paint_path(&*tank);
    }
  }
}
