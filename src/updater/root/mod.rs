// =============================================================================
//! - Root Updater for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-13
//! - Updated: 2023-07-28
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use self::events::RootUpdaterEventsAdapter;
use self::inputs::RootUpdaterInputsAdapter;
use crate::overlay::Overlay;
use crate::preparer::world::WorldPreparer;
use crate::root::Root;
use crate::root::RootMutator;
use crate::updater::options::OptionsUpdater;
use crate::updater::overlay::OverlayUpdater;
use crate::updater::world::WorldUpdater;
use com_croftsoft_lib_animation::frame_rater::updater::FrameRaterUpdater;
use com_croftsoft_lib_animation::frame_rater::FrameRater;
use com_croftsoft_lib_animation::metronome::delta::DeltaMetronome;
use com_croftsoft_lib_animation::metronome::updater::MetronomeUpdater;
use com_croftsoft_lib_role::Preparer;
use com_croftsoft_lib_role::Updater;
use core::cell::RefCell;
use std::rc::Rc;

use super::game::GameUpdater;

pub mod events;
pub mod inputs;

pub struct RootUpdater {
  child_updaters: Vec<Box<dyn Updater>>,
  world_preparer: WorldPreparer,
}

impl RootUpdater {
  pub fn new(
    frame_rater: Rc<RefCell<dyn FrameRater>>,
    root: Rc<dyn Root>,
    root_mutator: Rc<dyn RootMutator>,
  ) -> Self {
    let root_updater_events_adapter = Rc::new(RefCell::new(
      RootUpdaterEventsAdapter::new(root.get_events()),
    ));
    // TODO: Get rid of the adapters
    let root_updater_inputs_adapter = Rc::new(RefCell::new(
      RootUpdaterInputsAdapter::new(root.get_events(), root.get_inputs()),
    ));
    // TODO: Should OptionsMutator extend Options?
    let options = root.get_options();
    let overlay: Rc<RefCell<Overlay>> = root.get_overlay();
    let frame_rater_updater = FrameRaterUpdater::new(
      false,
      frame_rater.clone(),
      root_updater_inputs_adapter.clone(),
    );
    let game_updater = GameUpdater::new(root.clone(), root_mutator.clone());
    let options_updater = OptionsUpdater::new(
      root_updater_inputs_adapter.clone(),
      root_mutator.get_options_mutator(),
    );
    let overlay_updater = OverlayUpdater::new(
      root_updater_events_adapter.clone(),
      frame_rater,
      root_updater_inputs_adapter.clone(),
      options,
      overlay,
    );
    let configuration = root.get_configuration();
    let metronome = Rc::new(RefCell::new(DeltaMetronome {
      period_millis: configuration.update_period_millis_initial,
      time_millis_next_tick: 0.,
    }));
    let metronome_updater = MetronomeUpdater::new(
      root_updater_events_adapter,
      root_updater_inputs_adapter.clone(),
      metronome,
    );
    let world_updater = WorldUpdater::new(
      configuration,
      root_updater_inputs_adapter,
      root.clone(),
    );
    let child_updaters: Vec<Box<dyn Updater>> = vec![
      Box::new(metronome_updater),
      Box::new(options_updater),
      Box::new(game_updater),
      Box::new(world_updater),
      Box::new(frame_rater_updater),
      Box::new(overlay_updater),
    ];
    let world_preparer = WorldPreparer::new(root.get_world());
    Self {
      child_updaters,
      world_preparer,
    }
  }
}

impl Updater for RootUpdater {
  fn update(&mut self) {
    self.world_preparer.prepare();
    self
      .child_updaters
      .iter_mut()
      .for_each(|updater| updater.update());
  }
}
