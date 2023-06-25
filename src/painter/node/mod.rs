// =============================================================================
//! - Node Painter for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-24
//! - Updated: 2023-06-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::ai::state_space_node::StateSpaceNode;
use crate::ai::tank_operator::TankOperator;
use crate::constant::{
  NODE_STROKE_STYLE, TANK_FILL_STYLE_BLUE, TANK_FILL_STYLE_RED,
};
use crate::model::tank::Color;
use crate::options::Options;
use com_croftsoft_lib_role::Painter;
use core::cell::RefCell;
use core::f64::consts::TAU;
use std::collections::VecDeque;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct NodePainter {
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fill_style_blue: JsValue,
  fill_style_red: JsValue,
  options: Rc<RefCell<Options>>,
  stroke_style: JsValue,
  tank_operators: Rc<RefCell<VecDeque<Box<dyn TankOperator>>>>,
}

impl NodePainter {
  pub fn new(
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    options: Rc<RefCell<Options>>,
    tank_operators: Rc<RefCell<VecDeque<Box<dyn TankOperator>>>>,
  ) -> Self {
    let fill_style_blue: JsValue = JsValue::from_str(TANK_FILL_STYLE_BLUE);
    let fill_style_red: JsValue = JsValue::from_str(TANK_FILL_STYLE_RED);
    let stroke_style: JsValue = JsValue::from_str(NODE_STROKE_STYLE);
    Self {
      context,
      fill_style_blue,
      fill_style_red,
      options,
      stroke_style,
      tank_operators,
    }
  }

  fn paint_node(
    &self,
    tank_operator: &Box<dyn TankOperator>,
  ) -> Result<(), JsValue> {
    let tank = tank_operator.get_tank();
    let tank = tank.borrow();
    if tank.is_burning() || !tank.is_active() {
      return Ok(());
    }
    let context = self.context.borrow();
    context.save();
    let fill_style = match tank.get_color() {
      Color::RED => &self.fill_style_red,
      Color::BLUE => &self.fill_style_blue,
    };
    context.set_fill_style(fill_style);
    context.set_line_width(1.);
    context.set_stroke_style(&self.stroke_style);
    let state_space_nodes: Vec<StateSpaceNode> = tank_operator.get_nodes();
    state_space_nodes.iter().for_each(|state_space_node| {
      let point_2dd = state_space_node.get_point_xy();
      // TODO: show state space node heading
      context.begin_path();
      let _result = context.arc(point_2dd.x, point_2dd.y, 2., 0., TAU);
      context.fill();
      context.stroke();
    });
    context.restore();
    Ok(())
  }
}

impl Painter for NodePainter {
  fn paint(&mut self) {
    let options = self.options.borrow();
    if !options.node_display {
      return;
    }
    let tank_operators = self.tank_operators.borrow();
    for tank_operator in tank_operators.iter() {
      let _result = self.paint_node(tank_operator);
    }
  }
}
