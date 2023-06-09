// =============================================================================
//! - Update Rate Component for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-13
//! - Updated: 2023-06-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::Component;
use crate::inputs::Inputs;
use com_croftsoft_lib_animation::web_sys::add_change_handler_by_id;
use com_croftsoft_lib_role::{Initializer, Updater};
use core::cell::RefCell;
use futures::channel::mpsc::{TryRecvError, UnboundedReceiver};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement};

pub struct UpdateRateComponent {
  event_unbounded_receiver_option: Option<UnboundedReceiver<Event>>,
  id: String,
  inputs: Rc<RefCell<Inputs>>,
}

impl UpdateRateComponent {
  fn changed(&mut self) -> Option<Event> {
    let unbounded_receiver: &mut UnboundedReceiver<Event> =
      self.event_unbounded_receiver_option.as_mut()?;
    let result: Result<Option<Event>, TryRecvError> =
      unbounded_receiver.try_next();
    if let Ok(event_option) = result {
      return event_option;
    }
    None
  }

  pub fn new(
    id: &str,
    inputs: Rc<RefCell<Inputs>>,
  ) -> Self {
    Self {
      id: String::from(id),
      inputs,
      event_unbounded_receiver_option: None,
    }
  }
}

impl Component for UpdateRateComponent {
  fn make_html(&self) -> String {
    format!(
      "Display update rate <input id=\"{}\" type=\"checkbox\">",
      self.id
    )
  }
}

impl Initializer for UpdateRateComponent {
  fn initialize(&mut self) {
    self.event_unbounded_receiver_option = add_change_handler_by_id(&self.id);
  }
}

impl Updater for UpdateRateComponent {
  fn update(&mut self) {
    let event_option = self.changed();
    if let Some(event) = event_option {
      let event_target_option: Option<EventTarget> = event.target();
      if let Some(event_target) = event_target_option {
        let result: Result<HtmlInputElement, EventTarget> =
          event_target.dyn_into::<HtmlInputElement>();
        let html_input_element: HtmlInputElement = result.unwrap();
        self.inputs.borrow_mut().update_rate_display_change_requested =
          Some(html_input_element.checked());
      }
    }
  }
}
