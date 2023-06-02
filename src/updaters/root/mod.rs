// =============================================================================
//! - Root Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-13
//! - Updated: 2023-06-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::models::options::Options;
use crate::models::overlay::Overlay;
use crate::models::root::Root;
use crate::models::world::preparer::WorldPreparer;
use crate::updaters::options::OptionsUpdater;
use crate::updaters::options::OptionsUpdaterInputs;
use crate::updaters::overlay::OverlayUpdater;
use crate::updaters::overlay::OverlayUpdaterEvents;
use crate::updaters::overlay::OverlayUpdaterInputs;
use crate::updaters::overlay::OverlayUpdaterOptions;
use crate::updaters::world::WorldUpdater;
use com_croftsoft_lib_animation::frame_rater::updater::FrameRaterUpdater;
use com_croftsoft_lib_animation::frame_rater::updater::FrameRaterUpdaterInputs;
use com_croftsoft_lib_animation::frame_rater::FrameRater;
use com_croftsoft_lib_animation::metronome::delta::DeltaMetronome;
use com_croftsoft_lib_animation::metronome::updater::{
  MetronomeUpdater, MetronomeUpdaterEvents, MetronomeUpdaterInputs,
};
use com_croftsoft_lib_role::Preparer;
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::rc::Rc;

pub struct RootUpdaterConfiguration {
  pub update_period_millis_initial: f64,
}

pub trait RootUpdaterEvents {
  fn get_updated(&self) -> bool;
  fn get_time_to_update(&self) -> bool;
  fn get_update_period_millis_changed(&self) -> Option<f64>;
  fn set_time_to_update(&mut self);
  fn set_update_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  );
  fn set_updated(&mut self);
}

struct RootUpdaterEventsAdapter {
  events: Rc<RefCell<dyn RootUpdaterEvents>>,
}

impl RootUpdaterEventsAdapter {
  fn new(events: Rc<RefCell<dyn RootUpdaterEvents>>) -> Self {
    Self {
      events,
    }
  }
}

impl MetronomeUpdaterEvents for RootUpdaterEventsAdapter {
  fn set_period_millis_changed(
    &mut self,
    update_period_millis: f64,
  ) {
    self
      .events
      .borrow_mut()
      .set_update_period_millis_changed(update_period_millis);
  }

  fn set_tick(&mut self) {
    self.events.borrow_mut().set_time_to_update();
  }
}

impl OverlayUpdaterEvents for RootUpdaterEventsAdapter {
  fn set_updated(&mut self) {
    self.events.borrow_mut().set_updated();
  }
}

pub trait RootUpdaterInputs {
  fn get_current_time_millis(&self) -> f64;
  fn get_pause_change_requested(&self) -> Option<bool>;
  fn get_period_millis_change_requested(&self) -> Option<f64>;
  fn get_reset_requested(&self) -> bool;
  fn get_update_rate_display_change_requested(&self) -> Option<bool>;
}

struct RootUpdaterInputsAdapter {
  events: Rc<RefCell<dyn RootUpdaterEvents>>,
  inputs: Rc<RefCell<dyn RootUpdaterInputs>>,
}

impl RootUpdaterInputsAdapter {
  fn new(
    events: Rc<RefCell<dyn RootUpdaterEvents>>,
    inputs: Rc<RefCell<dyn RootUpdaterInputs>>,
  ) -> Self {
    Self {
      events,
      inputs,
    }
  }
}

impl FrameRaterUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_frame_rate_display_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_update_rate_display_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }

  fn get_update_period_millis_changed(&self) -> Option<f64> {
    self.events.borrow().get_update_period_millis_changed()
  }

  fn get_update_time_millis(&self) -> f64 {
    self.inputs.borrow().get_current_time_millis()
  }
}

impl MetronomeUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_current_time_millis(&self) -> f64 {
    self.inputs.borrow().get_current_time_millis()
  }

  fn get_period_millis_change_requested(&self) -> Option<f64> {
    self.inputs.borrow().get_period_millis_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }
}

impl OptionsUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_pause_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_pause_change_requested()
  }

  fn get_reset_requested(&self) -> bool {
    self.inputs.borrow().get_reset_requested()
  }

  // fn get_time_display_change_requested(&self) -> Option<bool> {
  //   self.inputs.borrow().get_time_display_change_requested()
  // }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }

  fn get_update_period_millis_changed(&self) -> Option<f64> {
    self.events.borrow().get_update_period_millis_changed()
  }

  fn get_update_rate_display_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_update_rate_display_change_requested()
  }

  fn get_update_time_millis(&self) -> f64 {
    self.inputs.borrow().get_current_time_millis()
  }
}

impl OverlayUpdaterInputs for RootUpdaterInputsAdapter {
  fn get_current_time_millis(&self) -> f64 {
    self.inputs.borrow().get_current_time_millis()
  }

  // fn get_pause_change_requested(&self) -> Option<bool> {
  //   self.inputs.borrow().get_pause_change_requested()
  // }

  // fn get_reset_requested(&self) -> bool {
  //   self.inputs.borrow().get_reset_requested()
  // }

  fn get_time_to_update(&self) -> bool {
    self.events.borrow().get_time_to_update()
  }

  fn get_update_rate_display_change_requested(&self) -> Option<bool> {
    self.inputs.borrow().get_update_rate_display_change_requested()
  }
}

pub trait RootUpdaterOptions {
  fn get_pause(&self) -> bool;
  // fn get_time_display(&self) -> bool;
  fn get_update_rate_display(&self) -> bool;
}

struct RootUpdaterOptionsAdapter {
  options: Rc<RefCell<dyn RootUpdaterOptions>>,
}

impl RootUpdaterOptionsAdapter {
  fn new(options: Rc<RefCell<dyn RootUpdaterOptions>>) -> Self {
    Self {
      options,
    }
  }
}

impl OverlayUpdaterOptions for RootUpdaterOptionsAdapter {
  fn get_pause(&self) -> bool {
    self.options.borrow().get_pause()
  }

  fn get_update_rate_display(&self) -> bool {
    self.options.borrow().get_update_rate_display()
  }
}

pub struct RootUpdater {
  child_updaters: Vec<Box<dyn Updater>>,
  world_preparer: WorldPreparer,
}

impl RootUpdater {
  pub fn new(
    configuration: RootUpdaterConfiguration,
    events: Rc<RefCell<dyn RootUpdaterEvents>>,
    frame_rater: Rc<RefCell<dyn FrameRater>>,
    inputs: Rc<RefCell<dyn RootUpdaterInputs>>,
    options: Rc<RefCell<Options>>,
    root_state: Rc<RefCell<Root>>,
  ) -> Self {
    let root_updater_events_adapter =
      Rc::new(RefCell::new(RootUpdaterEventsAdapter::new(events.clone())));
    let root_updater_inputs_adapter = Rc::new(RefCell::new(
      RootUpdaterInputsAdapter::new(events.clone(), inputs.clone()),
    ));
    let root_updater_options_adapter = Rc::new(RefCell::new(
      RootUpdaterOptionsAdapter::new(options.clone()),
    ));
    let overlay: Rc<RefCell<Overlay>> = root_state.borrow().overlay.clone();
    let frame_rater_updater = FrameRaterUpdater::new(
      false,
      frame_rater.clone(),
      root_updater_inputs_adapter.clone(),
    );
    let options_updater =
      OptionsUpdater::new(root_updater_inputs_adapter.clone(), options);
    let overlay_updater = OverlayUpdater::new(
      root_updater_events_adapter.clone(),
      frame_rater,
      root_updater_inputs_adapter.clone(),
      root_updater_options_adapter,
      overlay,
    );
    let metronome = Rc::new(RefCell::new(DeltaMetronome {
      period_millis: configuration.update_period_millis_initial,
      time_millis_next_tick: 0.,
    }));
    let metronome_updater = MetronomeUpdater::new(
      root_updater_events_adapter,
      root_updater_inputs_adapter,
      metronome,
    );
    let world_updater = WorldUpdater::new(root_state.borrow().world.clone());
    let child_updaters: Vec<Box<dyn Updater>> = vec![
      Box::new(metronome_updater),
      Box::new(options_updater),
      Box::new(world_updater),
      Box::new(frame_rater_updater),
      Box::new(overlay_updater),
    ];
    let world_preparer = WorldPreparer::new(root_state.borrow().world.clone());
    Self {
      child_updaters,
      world_preparer,
    }
  }
}

impl Updater for RootUpdater {
  fn update(&mut self) {
    self.world_preparer.prepare();
    self.child_updaters.iter_mut().for_each(|updater| updater.update());
  }
}
