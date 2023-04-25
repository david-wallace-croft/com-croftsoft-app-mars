// =============================================================================
//! - Tank state for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-29
//! - Updated: 2023-04-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::{Tank, TankAccessor, TankMutator};
use crate::ai::tank_console::TankConsole;
use crate::constants::{
  TANK_AMMO_INITIAL, TANK_AMMO_MAX,
  TANK_BODY_ROTATION_SPEED_RADIANS_PER_SECOND, TANK_DAMAGE_MAX, TANK_RADIUS,
  TANK_SPEED_METERS_PER_SECOND, TANK_TURRET_ROTATION_SPEED_RADIANS_PER_SECOND,
  TANK_Z,
};
use crate::engine::traits::{
  Color, Damageable, Impassable, Model, ModelAccessor, SpaceTester,
};
use crate::state::root::Root;
use com_croftsoft_core::math::geom::circle::Circle;
use com_croftsoft_core::math::geom::point_2dd::Point2DD;
use com_croftsoft_lib_animation::web_sys::log;
use core::f64::consts::{PI, TAU};
use core::f64::INFINITY;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct TankState {
  active: bool,
  ammo: usize,
  body_heading: f64,
  circle: Circle,
  color: Color,
  damage: f64,
  // TODO: was PointXY
  destination: Option<Point2DD>,
  dry_firing: bool,
  firing: bool,
  id: usize,
  sparking: bool,
  target_point: Point2DD,
  test_circle: Circle,
  turret_heading: f64,
  updated: bool,
}

impl TankState {
  pub fn initialize(
    &mut self,
    center_x: f64,
    center_y: f64,
  ) {
    self.ammo = TANK_AMMO_INITIAL;
    self.damage = 0.;
    self.prepare();
    self.active = true;
    self.updated = true;
    self.circle.center_x = center_x;
    self.circle.center_y = center_y;
  }

  pub fn new(
    center_x: f64,
    center_y: f64,
    color: Color,
    id: usize,
  ) -> Self {
    let circle: Circle = Circle {
      center_x: 0.,
      center_y: 0.,
      radius: TANK_RADIUS,
    };
    let test_circle: Circle = Circle {
      center_x: 0.,
      center_y: 0.,
      radius: TANK_RADIUS,
    };
    let mut tank: TankState = Self {
      active: false,
      ammo: 0,
      body_heading: 0.,
      circle,
      color,
      damage: 0.,
      destination: None,
      dry_firing: false,
      firing: false,
      id,
      sparking: false,
      // tank_operator: None,
      target_point: Point2DD::default(),
      test_circle,
      turret_heading: 0.,
      updated: false,
    };
    tank.initialize(center_x, center_y);
    // TODO: self.ammo_dumps = [];
    tank
  }

  pub fn set_ammo(
    &mut self,
    ammo: usize,
  ) {
    self.ammo = ammo;
  }

  pub fn update(
    &mut self,
    root: Rc<RefCell<Root>>,
    time_delta: f64,
  ) {
    // if let Some(tank_operator) = &self.tank_operator {
    //   log("TankState.update()");
    //   tank_operator.borrow_mut().update(time_delta);
    // }
    self.update_ammo();
    self.update_position(root, time_delta);
    self.update_turret_heading(time_delta);
  }

  // private update functions

  fn rotate_toward_heading(
    current_heading: f64,
    target_heading: f64,
    rotation_speed: f64,
  ) -> f64 {
    let mut new_heading;
    let delta_heading = target_heading - current_heading;
    if delta_heading < -PI {
      new_heading = current_heading + rotation_speed;
      if new_heading >= TAU {
        new_heading -= TAU;
      }
    } else if delta_heading < -rotation_speed {
      new_heading = current_heading - rotation_speed;
    } else if delta_heading <= rotation_speed {
      new_heading = target_heading;
    } else if delta_heading < PI {
      new_heading = current_heading + rotation_speed;
    } else {
      new_heading = current_heading - rotation_speed;
      if new_heading < 0. {
        new_heading += TAU
      }
    }
    new_heading
  }

  // private update methods

  fn update_ammo(&mut self) {
    if self.ammo >= TANK_AMMO_MAX {
      return;
    }
    let ammo_needed: usize = TANK_AMMO_MAX - self.ammo;
    // TODO
  }

  fn update_position(
    &mut self,
    root: Rc<RefCell<Root>>,
    time_delta: f64,
  ) {
    // log(&format!("destination {:?}", self.destination));
    if self.destination.is_none() {
      return;
    }
    // TODO: something better than unwrap
    let delta_x: f64 = self.destination.unwrap().x - self.circle.center_x;
    let delta_y: f64 = self.destination.unwrap().y - self.circle.center_y;
    // TODO: the following was remarked out in the old code
    // if ((Math.abs(deltaX) < 0.5) && (Math.abs(deltaY) < 0.5)) {
    //   return;
    // }
    // TODO: verify delta_y and delta_x not reversed
    let mut aim_heading: f64 = delta_y.atan2(delta_x);
    if aim_heading < 0. {
      aim_heading += TAU;
    }
    let new_body_heading: f64 = TankState::rotate_toward_heading(
      self.body_heading,
      aim_heading,
      time_delta * TANK_BODY_ROTATION_SPEED_RADIANS_PER_SECOND,
    );
    if new_body_heading != self.body_heading {
      self.updated = true;
      self.body_heading = new_body_heading;
    }
    if self.body_heading != aim_heading {
      return;
    }
    let mut move_x: f64 =
      time_delta * TANK_SPEED_METERS_PER_SECOND * self.body_heading.cos();
    let mut move_y: f64 =
      time_delta * TANK_SPEED_METERS_PER_SECOND * self.body_heading.sin();
    if move_x.abs() > delta_x.abs() {
      move_x = delta_x;
    }
    if move_y.abs() > delta_y.abs() {
      move_y = delta_y;
    }
    let old_x = self.circle.center_x;
    let old_y = self.circle.center_y;
    let new_x = self.circle.center_x + move_x;
    let new_y = self.circle.center_y + move_y;
    self.circle.center_x = new_x;
    self.circle.center_y = new_y;
    // TODO
    if root.borrow().is_blocked(&self.circle) {
      self.circle.center_x = old_x;
      self.circle.center_y = old_y;
      if root.borrow().is_blocked(&self.circle) {
        self.circle.center_x = new_x;
        self.circle.center_y = new_y;
        self.updated = true;
      }
    } else {
      // log(&format!("center updated {:?}", self.circle));
      self.updated = true;
    }
  }

  fn update_turret_heading(
    &mut self,
    time_delta: f64,
  ) {
    let center_x: f64 = self.circle.center_x;
    let center_y: f64 = self.circle.center_y;
    // TODO: check if atan2 arguments reversed
    let mut desired_turret_heading: f64 =
      (self.target_point.y - center_y).atan2(self.target_point.x - center_x);
    if desired_turret_heading < 0. {
      desired_turret_heading += TAU;
    }
    let new_turret_heading = TankState::rotate_toward_heading(
      self.turret_heading,
      desired_turret_heading,
      time_delta * TANK_TURRET_ROTATION_SPEED_RADIANS_PER_SECOND,
    );
    if new_turret_heading == self.turret_heading {
      return;
    }
    self.updated = true;
    self.turret_heading = new_turret_heading;
  }
}

impl Damageable for TankState {
  fn add_damage(
    &mut self,
    new_damage: f64,
  ) {
    if !self.active || new_damage == 0. {
      return;
    }
    self.updated = true;
    self.sparking = true;
    self.damage += new_damage;
    if self.damage > TANK_DAMAGE_MAX {
      self.active = false;
    }
  }
}

impl Impassable for TankState {}

impl Model for TankState {
  fn prepare(&mut self) {
    self.updated = false;
    self.firing = false;
    self.dry_firing = false;
    self.sparking = false;
  }

  fn set_center(
    &mut self,
    x: f64,
    y: f64,
  ) {
    self.circle.center_x = x;
    self.circle.center_y = y;
  }

  fn update(
    &mut self,
    root: Rc<RefCell<Root>>,
    time_delta: f64,
  ) {
    log("TankState.update()");
    if !self.active {
      return;
    }
    // let Some(tank_operator) = &self.tank_operator else { return; };
    // tank_operator.borrow_mut().update(time_delta);
    self.update_ammo();
    self.update_position(root, time_delta);
    self.update_turret_heading(time_delta);
  }
}

impl ModelAccessor for TankState {
  fn get_shape(
    &self,
    mut circle: Circle,
  ) -> Circle {
    circle.center_x = self.circle.center_x;
    circle.center_y = self.circle.center_y;
    circle.radius = self.circle.radius;
    circle
  }

  fn get_z(&self) -> f64 {
    TANK_Z
  }

  fn is_active(&self) -> bool {
    self.active
  }

  fn is_updated(&self) -> bool {
    self.updated
  }
}

impl SpaceTester for TankState {
  fn is_space_available(
    &self,
    // TODO: this was PointXY
    x: f64,
    y: f64,
  ) -> bool {
    // TODO
    true
  }
}

impl Tank for TankState {
  // fn get_tank_operator(&self) -> Rc<RefCell<dyn TankOperator>> {
  //   todo!()
  // }

  fn initialize(
    &mut self,
    center_x: f64,
    center_y: f64,
  ) {
    todo!()
  }

  // fn set_tank_operator(
  //   &mut self,
  //   tank_operator: Rc<RefCell<dyn TankOperator>>,
  // ) {
  //   self.tank_operator = Some(tank_operator);
  // }
}

impl TankAccessor for TankState {
  fn get_ammo(&self) -> usize {
    self.ammo
  }

  fn get_body_heading(&self) -> f64 {
    self.body_heading
  }

  fn get_color(&self) -> Color {
    self.color
  }

  fn get_damage(&self) -> f64 {
    self.damage
  }

  fn get_radius(&self) -> f64 {
    self.circle.radius
  }

  fn get_turret_heading(&self) -> f64 {
    self.turret_heading
  }

  fn is_dry_firing(&self) -> bool {
    self.dry_firing
  }

  fn is_firing(&self) -> bool {
    self.firing
  }

  fn is_sparking(&self) -> bool {
    self.sparking
  }
}

impl TankConsole for TankState {
  fn fire(&mut self) {
    if !self.active || self.firing || self.dry_firing {
      return;
    }
    self.updated = true;
    if self.ammo < 1 {
      self.dry_firing = true;
      return;
    }
    self.ammo -= 1;
    self.firing = true;
    let bullet_origin_x: f64 =
      self.circle.center_x + (TANK_RADIUS + 3.) * self.turret_heading.cos();
    let bullet_origin_y: f64 =
      self.circle.center_y + (TANK_RADIUS + 3.) * self.turret_heading.sin();
    // TODO: self.world.fire_bullet(
    //   bullet_origin_x, bullet_origin_y, turret_heading);
  }

  fn get_body_rotation_speed(&self) -> f64 {
    TANK_BODY_ROTATION_SPEED_RADIANS_PER_SECOND
  }

  fn get_center(
    &self,
    center: &mut Point2DD,
  ) {
    center.set_xy(self.circle.center_x, self.circle.center_y);
  }

  fn get_closest_ammo_dump_center(&self) -> Option<Point2DD> {
    // todo!()
    None
  }

  fn get_closest_enemy_tank_center(
    &self,
    tanks: Rc<RefCell<VecDeque<Rc<RefCell<TankState>>>>>,
  ) -> Option<Point2DD> {
    let mut closest_distance: f64 = INFINITY;
    let mut found = false;
    let tanks = tanks.borrow();
    let length = tanks.len();
    let tank_center = Point2DD::new(self.circle.center_x, self.circle.center_y);
    let mut enemy_tank_center = Point2DD::default();
    let mut closest_enemy_tank_center = Point2DD::default();
    for i in 0..length {
      let tank = tanks[i].borrow();
      if !tank.is_active() || tank.color == self.color {
        continue;
      }
      tank.get_center(&mut enemy_tank_center);
      let distance = tank_center.distance_to(&enemy_tank_center);
      if distance < closest_distance {
        found = true;
        closest_distance = distance;
        closest_enemy_tank_center.x = enemy_tank_center.x;
        closest_enemy_tank_center.y = enemy_tank_center.y;
      }
    }
    if found {
      return Some(closest_enemy_tank_center);
    }
    None
  }

  fn get_id(&self) -> usize {
    self.id
  }

  fn get_tank_speed(&self) -> f64 {
    TANK_SPEED_METERS_PER_SECOND
  }

  fn go(
    &mut self,
    destination: &Point2DD,
  ) {
    self.destination = Some(Point2DD::new(destination.x, destination.y));
  }

  fn rotate_turret(
    &mut self,
    target_point: &Option<Point2DD>,
  ) {
    if let Some(target_point) = target_point {
      self.target_point.set_xy_point(target_point);
    }
    // TODO: else if None, rotate turret forward maybe
  }
}

impl TankMutator for TankState {
  fn set_body_heading(
    &mut self,
    body_heading: f64,
  ) {
    self.body_heading = body_heading;
  }

  fn set_turret_heading(
    &mut self,
    turret_heading: f64,
  ) {
    self.turret_heading = turret_heading;
  }
}
