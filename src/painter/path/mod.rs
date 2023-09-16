// =============================================================================
//! - Path Painter for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-18
//! - Updated: 2023-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::ai::state_space_node::StateSpaceNode;
use crate::ai::tank_operator::TankOperator;
use crate::constant::{TANK_FILL_STYLE_BLUE, TANK_FILL_STYLE_RED, TANK_RADIUS};
use crate::model::tank::Color;
use crate::options::Options;
use com_croftsoft_lib_role::Painter;
use core::cell::RefCell;
use core::f64::consts::TAU;
use std::collections::VecDeque;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct PathPainter {
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fill_style_blue: JsValue,
  fill_style_red: JsValue,
  options: Rc<dyn Options>,
  tank_operators: Rc<RefCell<VecDeque<Box<dyn TankOperator>>>>,
}

impl PathPainter {
  pub fn new(
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    options: Rc<dyn Options>,
    tank_operators: Rc<RefCell<VecDeque<Box<dyn TankOperator>>>>,
  ) -> Self {
    let fill_style_blue: JsValue = JsValue::from_str(TANK_FILL_STYLE_BLUE);
    let fill_style_red: JsValue = JsValue::from_str(TANK_FILL_STYLE_RED);
    Self {
      context,
      fill_style_blue,
      fill_style_red,
      options,
      tank_operators,
    }
  }

  fn paint_path(
    &self,
    tank_operator: &dyn TankOperator,
  ) -> Result<(), JsValue> {
    let tank = tank_operator.get_tank();
    let tank = tank.borrow();
    if tank.is_burning() || !tank.is_active() {
      return Ok(());
    }
    let context = self.context.borrow();
    context.save();
    let stroke_style = match tank.get_color() {
      Color::RED => &self.fill_style_red,
      Color::BLUE => &self.fill_style_blue,
    };
    context.set_stroke_style(stroke_style);
    context.set_line_width(3.);
    let state_space_nodes: VecDeque<StateSpaceNode> = tank_operator.get_path();
    state_space_nodes.iter().for_each(|state_space_node| {
      let point_2dd = state_space_node.get_point_xy();
      // TODO: show state space node heading
      context.begin_path();
      let _result = context.arc(point_2dd.x, point_2dd.y, TANK_RADIUS, 0., TAU);
      context.stroke();
    });
    context.restore();
    Ok(())
  }
}

impl Painter for PathPainter {
  fn paint(&self) {
    if !self.options.get_path_display() {
      return;
    }
    let tank_operators = self.tank_operators.borrow();
    for tank_operator in tank_operators.iter() {
      let _result = self.paint_path(tank_operator.as_ref());
    }
  }
}
