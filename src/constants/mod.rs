// =============================================================================
//! - Constants for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-03-29
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use crate::state::configuration::Configuration;

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
pub const OBSTACLE_FILL_STYLE: &str = "#40826D";
pub const OBSTACLE_STROKE_STYLE: &str = "black";
pub const OBSTACLE_JERK_MAGNITUDE_MAX: f64 = 100.0;
pub const OBSTACLE_SPEED_MAX: f64 = 1_000.0;
pub const OBSTACLE_Z: f64 = 0.2;
pub const OVERLAY_FILL_STYLE: &str = "black";
pub const OVERLAY_REFRESH_PERIOD_MILLIS: f64 = 1_000.;
pub const TANK_Z: f64 = 1.;
pub const TIME_DELTA: f64 = 1. / UPDATES_PER_SECOND;
pub const UPDATE_PERIOD_MILLIS: f64 = MILLIS_PER_SECOND / UPDATES_PER_SECOND;
pub const UPDATES_PER_SECOND: f64 = 60.;

// TODO
// 046         /** radians per second */
// 047         private static final double  TURRET_ROTATION_SPEED
// 048           = 2.0 * Math.PI / 2.0;
// 049
// 050         /** radians per second */
// 051         private static final double  BODY_ROTATION_SPEED
// 052           = 2.0 * Math.PI / 6.0;
// 053
// 054         /** meters per second */
// 055         private static final double  TANK_SPEED = 30.0;
// 056
// 057         /** meters per second */
// 058         private static final double  RADIUS = 25.0;
// 059
// 060         private static final int     MAX_AMMO = 30;
// 061
// 062         private static final double  DAMAGE_MAX = 2.0;
// 063
// 064         //
// 065
// 066         private static final int  INITIAL_AMMO = ( int ) DAMAGE_MAX + 1;
