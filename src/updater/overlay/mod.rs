// =============================================================================
//! - Overlay Updater for CroftSoft Mars
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

use crate::constant::OVERLAY_REFRESH_PERIOD_MILLIS;
use crate::overlay::Overlay;
use com_croftsoft_lib_animation::frame_rater::FrameRater;
use com_croftsoft_lib_animation::metronome::delta::DeltaMetronome;
use com_croftsoft_lib_animation::metronome::Metronome;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub trait OverlayUpdaterEvents {
  fn set_updated(&mut self);
}

pub trait OverlayUpdaterInputs {
  fn get_current_time_millis(&self) -> f64;
  // fn get_pause_change_requested(&self) -> Option<bool>;
  // fn get_reset_requested(&self) -> bool;
  fn get_time_to_update(&self) -> bool;
  fn get_update_rate_display_change_requested(&self) -> Option<bool>;
}

pub trait OverlayUpdaterOptions {
  fn get_pause(&self) -> bool;
  fn get_update_rate_display(&self) -> bool;
}

pub struct OverlayUpdater {
  events: Rc<RefCell<dyn OverlayUpdaterEvents>>,
  frame_rater: Rc<RefCell<dyn FrameRater>>,
  inputs: Rc<RefCell<dyn OverlayUpdaterInputs>>,
  metronome: DeltaMetronome,
  options: Rc<RefCell<dyn OverlayUpdaterOptions>>,
  overlay: Rc<RefCell<Overlay>>,
}

impl OverlayUpdater {
  fn make_update_rate_string(&self) -> String {
    format!(
      "Updates per second: {:.3}",
      self.frame_rater.borrow().get_frames_per_second_sampled()
    )
  }

  pub fn new(
    events: Rc<RefCell<dyn OverlayUpdaterEvents>>,
    frame_rater: Rc<RefCell<dyn FrameRater>>,
    inputs: Rc<RefCell<dyn OverlayUpdaterInputs>>,
    options: Rc<RefCell<dyn OverlayUpdaterOptions>>,
    overlay: Rc<RefCell<Overlay>>,
  ) -> Self {
    let metronome = DeltaMetronome {
      period_millis: OVERLAY_REFRESH_PERIOD_MILLIS,
      time_millis_next_tick: 0.,
    };
    Self {
      events,
      frame_rater,
      inputs,
      metronome,
      options,
      overlay,
    }
  }

  fn update_overlay(&self) {
    let options: Ref<dyn OverlayUpdaterOptions> = self.options.borrow();
    let mut overlay: RefMut<Overlay> = self.overlay.borrow_mut();
    if !options.get_pause() && options.get_update_rate_display() {
      overlay.update_rate_string = self.make_update_rate_string();
      // TODO: Only set updated to true when the overlay data changes
      self.events.borrow_mut().set_updated();
    }
  }
}

impl Updater for OverlayUpdater {
  fn update(&mut self) {
    let inputs: Ref<dyn OverlayUpdaterInputs> = self.inputs.borrow();
    if inputs.get_update_rate_display_change_requested().is_some()
    // inputs.get_bug_requested().is_some()
    //   || inputs.get_pause_change_requested().is_some()
    //   || inputs.get_reset_requested()
    //   || inputs.get_time_display_change_requested().is_some()
    {
      self.update_overlay();
      return;
    }
    if inputs.get_time_to_update() {
      let current_time_millis: f64 = inputs.get_current_time_millis();
      if self.metronome.tick(current_time_millis) {
        self.update_overlay();
      }
    }
  }
}
