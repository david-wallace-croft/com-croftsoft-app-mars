// =============================================================================
//! - Path Component for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-06-22
//! - Updated: 2023-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::Component;
use crate::inputs::Inputs;
use com_croftsoft_lib_animation::web_sys::add_change_handler_by_id;
use com_croftsoft_lib_role::{InitializerMut, UpdaterMut};
use core::cell::RefCell;
use futures::channel::mpsc::{TryRecvError, UnboundedReceiver};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement};

pub struct PathComponent {
  event_unbounded_receiver_option: Option<UnboundedReceiver<Event>>,
  id: String,
  inputs: Rc<RefCell<Inputs>>,
}

impl PathComponent {
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

impl Component for PathComponent {
  fn make_html(&self) -> String {
    format!("Display paths <input id=\"{}\" type=\"checkbox\">", self.id)
  }
}

impl InitializerMut for PathComponent {
  fn initialize(&mut self) {
    self.event_unbounded_receiver_option = add_change_handler_by_id(&self.id);
  }
}

impl UpdaterMut for PathComponent {
  fn update(&mut self) {
    let event_option = self.changed();
    if let Some(event) = event_option {
      let event_target_option: Option<EventTarget> = event.target();
      if let Some(event_target) = event_target_option {
        let result: Result<HtmlInputElement, EventTarget> =
          event_target.dyn_into::<HtmlInputElement>();
        let html_input_element: HtmlInputElement = result.unwrap();
        self.inputs.borrow_mut().path_display_change_requested =
          Some(html_input_element.checked());
      }
    }
  }
}
