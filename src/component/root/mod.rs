// =============================================================================
//! - Root Component for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-07-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::node::NodeComponent;
use super::path::PathComponent;
use super::pause::PauseComponent;
use super::reset::ResetComponent;
use super::Component;
use crate::options::Options;
use crate::root::Root;
// use super::speed::SpeedComponent;
use super::canvas::CanvasComponent;
use super::update_rate::UpdateRateComponent;
use crate::events::Events;
use crate::inputs::Inputs;
use com_croftsoft_lib_animation::web_sys::get_window;
use com_croftsoft_lib_role::{Initializer, Painter, Updater};
use core::cell::RefCell;
use std::rc::Rc;
use web_sys::{Document, HtmlCollection};

pub struct RootComponent {
  canvas_component: Rc<RefCell<CanvasComponent>>,
  components: [Rc<RefCell<dyn Component>>; 6],
  // events: Rc<RefCell<Events>>,
  node_component: Rc<RefCell<NodeComponent>>,
  path_component: Rc<RefCell<PathComponent>>,
  pause_component: Rc<RefCell<PauseComponent>>,
  reset_component: Rc<RefCell<ResetComponent>>,
  update_rate_component: Rc<RefCell<UpdateRateComponent>>,
  // pause_component: Rc<RefCell<PauseComponent>>,
}

impl RootComponent {
  pub fn new(
    // TODO
    _events: Rc<RefCell<Events>>,
    // TODO: do something with the ID
    _id: &str,
    inputs: Rc<RefCell<Inputs>>,
    options: Rc<RefCell<Options>>,
    root: Rc<dyn Root>,
  ) -> Self {
    let canvas_component = Rc::new(RefCell::new(CanvasComponent::new(
      "canvas",
      inputs.clone(),
      options,
      root,
    )));
    let node_component =
      Rc::new(RefCell::new(NodeComponent::new("node", inputs.clone())));
    let path_component =
      Rc::new(RefCell::new(PathComponent::new("path", inputs.clone())));
    let pause_component =
      Rc::new(RefCell::new(PauseComponent::new("pause", inputs.clone())));
    let reset_component =
      Rc::new(RefCell::new(ResetComponent::new("reset", inputs.clone())));
    let update_rate_component = Rc::new(RefCell::new(
      UpdateRateComponent::new("update-rate", inputs),
    ));
    // let pause_component =
    //   Rc::new(RefCell::new(PauseComponent::new("pause", inputs.clone())));
    // let reset_component =
    //   Rc::new(RefCell::new(ResetComponent::new("reset", inputs.clone())));
    // let speed_component =
    //   Rc::new(RefCell::new(SpeedComponent::new("speed", inputs.clone())));
    let components: [Rc<RefCell<dyn Component>>; 6] = [
      canvas_component.clone(),
      node_component.clone(),
      path_component.clone(),
      pause_component.clone(),
      reset_component.clone(),
      update_rate_component.clone(),
      // frame_rate_component.clone(),
      // pause_component.clone(),
      // reset_component.clone(),
    ];
    Self {
      canvas_component,
      components,
      // events,
      node_component,
      path_component,
      pause_component,
      reset_component,
      update_rate_component,
    }
  }
}

impl Component for RootComponent {
  fn make_html(&self) -> String {
    let canvas_html: String = self.canvas_component.borrow().make_html();
    let node_html: String = self.node_component.borrow().make_html();
    let path_html: String = self.path_component.borrow().make_html();
    let pause_html: String = self.pause_component.borrow().make_html();
    let reset_html: String = self.reset_component.borrow().make_html();
    let update_rate_html: String =
      self.update_rate_component.borrow().make_html();
    // let pause_html: String = self.pause_component.borrow().make_html();
    // TODO: Assemble this from an HTML template
    [
      String::from("<div id=\"root\">"),
      canvas_html,
      String::from("<br>"),
      reset_html,
      // speed_html,
      node_html,
      path_html,
      update_rate_html,
      // time_html,
      pause_html,
      String::from("</div>"),
    ]
    .join("\n")
  }
}

impl Initializer for RootComponent {
  fn initialize(&mut self) {
    let document: Document = get_window().unwrap().document().unwrap();
    let html_collection: HtmlCollection =
      document.get_elements_by_tag_name("com-croftsoft-app-mars");
    let element_option = html_collection.item(0);
    let element = element_option.unwrap();
    let root_html: String = self.make_html();
    // TODO: Remove existing child nodes
    let _result = element.insert_adjacent_html("afterbegin", &root_html);
    self
      .components
      .iter()
      .for_each(|component| component.borrow_mut().initialize());
  }
}

impl Painter for RootComponent {
  fn paint(&mut self) {
    // TODO
    // if !self.events.borrow().updated {
    //   return;
    // }
    self.canvas_component.borrow_mut().paint();
  }
}

impl Updater for RootComponent {
  fn update(&mut self) {
    self
      .components
      .iter()
      .for_each(|component| component.borrow_mut().update());
  }
}
