// =============================================================================
//! - Constants for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-11
//! - Updated: 2023-03-11
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub static INFO: &str =
  "CroftSoft Mars v0.0.0-SNAPSHOT Copyright 2023 CroftSoft Inc";

pub const FILL_STYLE_BACKGROUND: &str = "black";
pub const MILLIS_PER_SECOND: f64 = 1_000.;
pub const OVERLAY_REFRESH_PERIOD_MILLIS: f64 = 1_000.;
pub const UPDATES_PER_SECOND: f64 = 1.;
pub const UPDATE_PERIOD_MILLIS: f64 = MILLIS_PER_SECOND / UPDATES_PER_SECOND;
