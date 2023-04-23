// =============================================================================
//! - Constants for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-04-23
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::state::configuration::Configuration;
use core::f64::consts::TAU;

pub const A_STAR_DIRECTIONS: usize = 8;
pub const A_STAR_LOOPS: usize = 100;
pub const A_STAR_STEP_SIZE: f64 = 10.;
pub const BACKGROUND_FILL_STYLE: &str = "rgb(255, 152, 109)";
pub const CONFIGURATION: Configuration = Configuration {
  update_period_millis_initial: UPDATE_PERIOD_MILLIS,
};
pub static FONT: &str = "bold 17px monospace";
pub static INFO: &str = "CroftSoft Mars \
  v0.0.1-SNAPSHOT \
  Copyright 2023 \
  CroftSoft Inc";
pub const MILLIS_PER_SECOND: f64 = 1_000.;
pub const OBSTACLE_RANDOM_PLACEMENT_ATTEMPTS_MAX: usize = 10;
pub const OBSTACLE_COUNT: usize = 6;
pub const OBSTACLE_FILL_STYLE: &str = "#40826D";
pub const OBSTACLE_JERK_MAGNITUDE_MAX: f64 = 100.;
pub const OBSTACLE_RADIUS_MAX: f64 = 60.;
pub const OBSTACLE_RADIUS_MIN: f64 = 10.;
pub const OBSTACLE_SPEED_MAX: f64 = 1_000.;
pub const OBSTACLE_STROKE_STYLE: &str = "black";
pub const OBSTACLE_Z: f64 = 0.2;
pub const OVERLAY_FILL_STYLE: &str = "black";
pub const OVERLAY_REFRESH_PERIOD_MILLIS: f64 = 1_000.;
pub const TANK_AMMO_INITIAL: usize = TANK_DAMAGE_MAX as usize + 1;
pub const TANK_AMMO_MAX: usize = 30;
pub const TANK_BODY_ROTATION_SPEED_RADIANS_PER_SECOND: f64 = TAU / 6.;
pub const TANK_DAMAGE_MAX: f64 = 2.;
pub const TANK_DRIFT_PROBABILITY: f64 = 0.1;
pub const TANK_FILL_STYLE: &str = "rgb(128, 128, 64)";
pub const TANK_FIRING_PROBABILITY: f64 = 1.;
pub const TANK_RADIUS: f64 = 25.;
pub const TANK_SPEED_METERS_PER_SECOND: f64 = 30.;
pub const TANK_STROKE_STYLE: &str = "black";
pub const TANK_TURRET_ROTATION_SPEED_RADIANS_PER_SECOND: f64 = TAU / 2.;
pub const TANK_Z: f64 = 1.;
pub const TIME_DELTA: f64 = 1. / UPDATES_PER_SECOND;
pub const UPDATE_PERIOD_MILLIS: f64 = MILLIS_PER_SECOND / UPDATES_PER_SECOND;
pub const UPDATES_PER_SECOND: f64 = 60.;
