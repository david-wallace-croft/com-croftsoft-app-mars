// =============================================================================
//! - Root Component for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-03-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::engine::traits::Component;
// use super::canvas::CanvasComponent;
// use super::frame_rate::FrameRateComponent;
// use super::pause::PauseComponent;
// use super::reset::ResetComponent;
// use super::speed::SpeedComponent;
// use crate::engine::traits::Component;
// use crate::messages::events::Events;
// use crate::messages::inputs::Inputs;
// use crate::models::options::Options;
// use crate::models::root::Root;
use super::canvas::CanvasComponent;
use super::update_rate::UpdateRateComponent;
use crate::messages::events::Events;
use crate::messages::inputs::Inputs;
use crate::state::options::Options;
use crate::state::root::Root;
use com_croftsoft_lib_animation::web_sys::{get_window, log};
use com_croftsoft_lib_role::{Initializer, Painter, Updater};
use core::cell::RefCell;
use std::rc::Rc;
use web_sys::{Document, HtmlCollection};

pub struct RootComponent {
  canvas_component: Rc<RefCell<CanvasComponent>>,
  components: [Rc<RefCell<dyn Component>>; 2],
  events: Rc<RefCell<Events>>,
  update_rate_component: Rc<RefCell<UpdateRateComponent>>,
  // pause_component: Rc<RefCell<PauseComponent>>,
  // reset_component: Rc<RefCell<ResetComponent>>,
}

impl RootComponent {
  pub fn new(
    events: Rc<RefCell<Events>>,
    // TODO: do something with the ID
    _id: &str,
    inputs: Rc<RefCell<Inputs>>,
    options: Rc<RefCell<Options>>,
    root_state: Rc<RefCell<Root>>,
  ) -> Self {
    let canvas_component = Rc::new(RefCell::new(CanvasComponent::new(
      "canvas",
      inputs.clone(),
      options,
      root_state,
    )));
    let update_rate_component = Rc::new(RefCell::new(
      UpdateRateComponent::new("update-rate", inputs),
    ));
    // let pause_component =
    //   Rc::new(RefCell::new(PauseComponent::new("pause", inputs.clone())));
    // let reset_component =
    //   Rc::new(RefCell::new(ResetComponent::new("reset", inputs.clone())));
    // let speed_component =
    //   Rc::new(RefCell::new(SpeedComponent::new("speed", inputs.clone())));
    let components: [Rc<RefCell<dyn Component>>; 2] = [
      canvas_component.clone(),
      update_rate_component.clone(),
      // frame_rate_component.clone(),
      // pause_component.clone(),
      // reset_component.clone(),
    ];
    Self {
      canvas_component,
      components,
      events,
      update_rate_component,
      // pause_component,
      // reset_component,
    }
  }
}

impl Component for RootComponent {
  fn make_html(&self) -> String {
    let canvas_html: String = self.canvas_component.borrow().make_html();
    let update_rate_html: String =
      self.update_rate_component.borrow().make_html();
    // let pause_html: String = self.pause_component.borrow().make_html();
    // let reset_html: String = self.reset_component.borrow().make_html();
    // TODO: Assemble this from an HTML template
    [
      String::from("<div id=\"root\">"),
      canvas_html,
      String::from("<br>"),
      // reset_html,
      // String::from("<br>"),
      // speed_html,
      update_rate_html,
      // time_html,
      // pause_html,
      String::from("</div>"),
    ]
    .join("\n")
  }
}

impl Initializer for RootComponent {
  fn initialize(&mut self) {
    log("initialize");
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
