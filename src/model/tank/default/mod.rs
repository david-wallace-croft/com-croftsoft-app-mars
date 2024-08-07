// =============================================================================
//! - Tank state for CroftSoft Mars
//!
//! # Metadata
//! - Copyright: &copy; 2023-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-29
//! - Updated: 2024-07-29
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use self::state::State;
use super::{Color, Tank, TankAccessor};
use crate::ai::tank_operator::TankOperator;
use crate::constant::{
  TANK_AMMO_INITIAL, TANK_AMMO_MAX,
  TANK_BODY_ROTATION_SPEED_RADIANS_PER_SECOND, TANK_BURNING_DURATION_SECONDS,
  TANK_DAMAGE_MAX, TANK_RADIUS, TANK_RELOAD_TIME_SECONDS,
  TANK_SPARKING_DURATION_SECONDS, TANK_SPEED_METERS_PER_SECOND,
  TANK_TREAD_LENGTH, TANK_TURRET_ROTATION_SPEED_RADIANS_PER_SECOND, TANK_Z,
};
use crate::model::bullet::Bullet;
use crate::model::{Damageable, Model, ModelAccessor};
use crate::world::factory::WorldFactory;
use crate::world::World;
use com_croftsoft_core::math::geom::circle::{Circle, CircleAccessor};
use com_croftsoft_core::math::geom::point_2dd::Point2DD;
use com_croftsoft_core::math::geom::point_xy::PointXY;
use com_croftsoft_lib_role::PreparerMut;
use core::f64::consts::{PI, TAU};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

pub mod state;

pub struct DefaultTank {
  ammo: usize,
  body_heading: f64,
  burning_time_remaining: f64,
  circle: Circle,
  color: Color,
  damage: f64,
  // TODO: was PointXY
  destination: Option<Point2DD>,
  dry_firing: bool,
  factory: Weak<dyn WorldFactory>,
  fire_requested: bool,
  firing: bool,
  id: usize,
  sparking_time_remaining: f64,
  state: State,
  target_point: Point2DD,
  time_since_last_fired: f64,
  tread_offset_left: f64,
  tread_offset_right: f64,
  turret_heading: f64,
  updated: bool,
  world: Weak<dyn World>,
}

impl DefaultTank {
  pub fn initialize(
    &mut self,
    center_x: f64,
    center_y: f64,
  ) {
    self.ammo = TANK_AMMO_INITIAL;
    self.damage = 0.;
    self.prepare();
    self.updated = true;
    self.circle.center_x = center_x;
    self.circle.center_y = center_y;
  }

  pub fn new(
    center_x: f64,
    center_y: f64,
    color: Color,
    factory: Weak<dyn WorldFactory>,
    id: usize,
    world: Weak<dyn World>,
  ) -> Self {
    let circle: Circle = Circle {
      center_x: 0.,
      center_y: 0.,
      radius: TANK_RADIUS,
    };
    let mut tank: DefaultTank = Self {
      ammo: 0,
      body_heading: 0.,
      burning_time_remaining: 0.,
      circle,
      color,
      damage: 0.,
      destination: None,
      dry_firing: false,
      factory,
      fire_requested: false,
      firing: false,
      id,
      sparking_time_remaining: 0.,
      state: State::default(),
      target_point: Point2DD::default(),
      time_since_last_fired: 0.,
      tread_offset_left: 0.,
      tread_offset_right: 0.,
      turret_heading: 0.,
      updated: false,
      world,
    };
    tank.initialize(center_x, center_y);
    tank
  }

  // private update functions

  fn is_turning_right(
    body_heading_new: f64,
    body_heading_old: f64,
  ) -> bool {
    if body_heading_old > body_heading_new {
      body_heading_old - body_heading_new > PI
    } else {
      body_heading_new - body_heading_old < PI
    }
  }

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
    let mut ammo_needed: usize = TANK_AMMO_MAX - self.ammo;
    let ammo_dumps = self.world.upgrade().unwrap().get_ammo_dumps();
    for ammo_dump in ammo_dumps.borrow_mut().iter_mut() {
      if !ammo_dump.contains(self.circle.center_x, self.circle.center_y) {
        continue;
      }
      let dump_ammo = ammo_dump.get_ammo();
      if ammo_needed as f64 <= dump_ammo {
        self.ammo = TANK_AMMO_MAX;
        ammo_dump.set_ammo(dump_ammo - ammo_needed as f64);
        break;
      } else {
        self.ammo += dump_ammo as usize;
        ammo_dump.set_ammo(dump_ammo - (dump_ammo as usize) as f64);
        ammo_needed = TANK_AMMO_MAX - self.ammo;
      }
    }
  }

  fn update_fire(
    &mut self,
    time_delta: f64,
  ) {
    self.time_since_last_fired += time_delta;
    if !self.fire_requested {
      return;
    }
    self.fire_requested = false;
    if self.firing || self.dry_firing {
      return;
    }
    self.updated = true;
    if self.ammo < 1 || self.time_since_last_fired < TANK_RELOAD_TIME_SECONDS {
      self.dry_firing = true;
      return;
    }
    self.ammo -= 1;
    self.firing = true;
    self.time_since_last_fired = 0.;
    let bullet_origin_x: f64 =
      self.circle.center_x + (TANK_RADIUS + 3.) * self.turret_heading.cos();
    let bullet_origin_y: f64 =
      self.circle.center_y + (TANK_RADIUS + 3.) * self.turret_heading.sin();
    let bullet: Box<dyn Bullet> = self.factory.upgrade().unwrap().make_bullet(
      self.turret_heading,
      bullet_origin_x,
      bullet_origin_y,
    );
    self.world.upgrade().unwrap().add_bullet(bullet);
  }

  fn update_position(
    &mut self,
    time_delta: f64,
  ) {
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
    let mut aim_heading: f64 = delta_y.atan2(delta_x);
    if aim_heading < 0. {
      aim_heading += TAU;
    }
    let body_heading_old: f64 = self.body_heading;
    let body_heading_new: f64 = DefaultTank::rotate_toward_heading(
      self.body_heading,
      aim_heading,
      time_delta * TANK_BODY_ROTATION_SPEED_RADIANS_PER_SECOND,
    );
    if body_heading_new != self.body_heading {
      self.updated = true;
      self.body_heading = body_heading_new;
    }
    let center_old = self.circle.get_center_point_2dd();
    if self.body_heading != aim_heading {
      let center_new = self.circle.get_center_point_2dd();
      self.update_tread_offsets(
        body_heading_new,
        body_heading_old,
        center_new,
        center_old,
        time_delta,
      );
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
    if self
      .world
      .upgrade()
      .unwrap()
      .is_blocked_by_impassable(&self.circle)
    {
      self.circle.center_x = old_x;
      self.circle.center_y = old_y;
      if self
        .world
        .upgrade()
        .unwrap()
        .is_blocked_by_impassable(&self.circle)
      {
        self.circle.center_x = new_x;
        self.circle.center_y = new_y;
        self.updated = true;
      }
    } else {
      // log(&format!("center updated {:?}", self.circle));
      self.updated = true;
    }
    let center_new = self.circle.get_center_point_2dd();
    self.update_tread_offsets(
      body_heading_new,
      body_heading_old,
      center_new,
      center_old,
      time_delta,
    );
  }

  fn update_tread_offsets(
    &mut self,
    body_heading_new: f64,
    body_heading_old: f64,
    center_new: Point2DD,
    center_old: Point2DD,
    time_delta: f64,
  ) {
    let mut tread_direction_left = 1.;
    let mut tread_direction_right = 1.;
    if (body_heading_new - body_heading_old).abs() < TAU / 1000. {
      if center_new.distance_xy(&center_old) < 0.25 {
        return;
      }
    } else if DefaultTank::is_turning_right(body_heading_new, body_heading_old)
    {
      tread_direction_right = -1.;
    } else {
      tread_direction_left = -1.;
    }
    let tread_delta_left =
      time_delta * TANK_SPEED_METERS_PER_SECOND * tread_direction_left;
    let tread_delta_right =
      time_delta * TANK_SPEED_METERS_PER_SECOND * tread_direction_right;
    self.tread_offset_left =
      (self.tread_offset_left + tread_delta_left) % TANK_TREAD_LENGTH;
    if self.tread_offset_left < 0. {
      self.tread_offset_left += TANK_TREAD_LENGTH;
    }
    self.tread_offset_right =
      (self.tread_offset_right + tread_delta_right) % TANK_TREAD_LENGTH;
    if self.tread_offset_right < 0. {
      self.tread_offset_right += TANK_TREAD_LENGTH;
    }
    self.updated = true;
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
    let new_turret_heading = DefaultTank::rotate_toward_heading(
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

impl Damageable for DefaultTank {
  fn add_damage(
    &mut self,
    new_damage: f64,
  ) {
    if new_damage <= 0. {
      return;
    }
    match &mut self.state {
      State::Burning(_) | State::Inactive => (),
      State::Nominal(state_operator) => {
        self.updated = true;
        self.damage += new_damage;
        if self.damage > TANK_DAMAGE_MAX {
          self.state = state_operator.to_burning();
          self.burning_time_remaining = TANK_BURNING_DURATION_SECONDS;
        } else {
          self.state = state_operator.to_sparking();
          self.sparking_time_remaining = TANK_SPARKING_DURATION_SECONDS;
        }
      },
      State::Sparking(state_operator) => {
        self.updated = true;
        self.damage += new_damage;
        if self.damage > TANK_DAMAGE_MAX {
          self.state = state_operator.to_burning();
          self.burning_time_remaining = TANK_BURNING_DURATION_SECONDS;
        } else {
          self.sparking_time_remaining = TANK_SPARKING_DURATION_SECONDS;
        }
      },
    }
  }
}

impl Model for DefaultTank {
  fn update(
    &mut self,
    time_delta: f64,
  ) {
    match &mut self.state {
      State::Burning(state_operator) => {
        self.burning_time_remaining -= time_delta;
        if self.burning_time_remaining <= 0. {
          self.state = state_operator.to_inactive();
        }
      },
      State::Inactive => (),
      State::Nominal(_) => {
        self.update_ammo();
        self.update_position(time_delta);
        self.update_turret_heading(time_delta);
        self.update_fire(time_delta);
      },
      State::Sparking(state_operator) => {
        self.sparking_time_remaining -= time_delta;
        if self.sparking_time_remaining <= 0. {
          self.state = state_operator.to_nominal();
        }
      },
    }
  }
}

impl ModelAccessor for DefaultTank {
  fn contains(
    &self,
    x: f64,
    y: f64,
  ) -> bool {
    self.circle.contains(x, y)
  }

  fn get_circle(&self) -> Circle {
    self.circle
  }

  fn get_id(&self) -> usize {
    self.id
  }

  fn get_z(&self) -> f64 {
    TANK_Z
  }

  fn intersects_circle(
    &self,
    circle: &dyn CircleAccessor,
  ) -> bool {
    self.circle.intersects_circle(circle)
  }

  fn is_active(&self) -> bool {
    !matches!(self.state, State::Inactive)
  }

  fn is_updated(&self) -> bool {
    self.updated
  }
}

impl PreparerMut for DefaultTank {
  fn prepare(&mut self) {
    self.updated = false;
    self.firing = false;
    self.dry_firing = false;
  }
}

impl Tank for DefaultTank {
  // moved from TankConsole
  fn fire(&mut self) {
    self.fire_requested = true;
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

  fn set_ammo(
    &mut self,
    ammo: usize,
  ) {
    self.ammo = ammo;
  }

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

impl TankAccessor for DefaultTank {
  fn get_ammo(&self) -> usize {
    self.ammo
  }

  fn get_body_heading(&self) -> f64 {
    self.body_heading
  }

  fn get_body_rotation_speed(&self) -> f64 {
    TANK_BODY_ROTATION_SPEED_RADIANS_PER_SECOND
  }

  fn get_center(&self) -> Point2DD {
    Point2DD::new(self.circle.center_x, self.circle.center_y)
  }

  fn get_closest_ammo_dump_circle(&self) -> Option<Circle> {
    let mut closest_ammo_dump_circle: Option<Circle> = None;
    let tank_center = self.get_center();
    let mut closest_distance: f64 = f64::INFINITY;
    let world = &self.world.upgrade().unwrap();
    let ammo_dumps = world.get_ammo_dumps();
    for ammo_dump in ammo_dumps.borrow().iter() {
      let ammo_dump_circle = ammo_dump.get_circle();
      let ammo_dump_center = ammo_dump_circle.get_center_point_2dd();
      let distance: f64 = tank_center.distance_to(&ammo_dump_center);
      if distance < closest_distance {
        closest_distance = distance;
        closest_ammo_dump_circle = Some(ammo_dump_circle);
      }
    }
    closest_ammo_dump_circle
  }

  fn get_closest_enemy_tank_circle(
    &self,
    tank_operators: Rc<RefCell<VecDeque<Box<dyn TankOperator>>>>,
  ) -> Option<Circle> {
    let mut closest_distance: f64 = f64::INFINITY;
    let mut found = false;
    let tank_operators = tank_operators.borrow();
    let length = tank_operators.len();
    let tank_center = Point2DD::new(self.circle.center_x, self.circle.center_y);
    let mut closest_enemy_tank_circle = Circle::default();
    for i in 0..length {
      let tank = tank_operators[i].get_tank();
      let tank = tank.borrow();
      if tank.is_burning()
        || !tank.is_active()
        || tank.get_color() == self.color
      {
        continue;
      }
      let enemy_tank_center = tank.get_center();
      let distance = tank_center.distance_to(&enemy_tank_center);
      if distance < closest_distance {
        found = true;
        closest_distance = distance;
        closest_enemy_tank_circle = tank.get_circle();
      }
    }
    if found {
      return Some(closest_enemy_tank_circle);
    }
    None
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

  fn get_tank_speed(&self) -> f64 {
    TANK_SPEED_METERS_PER_SECOND
  }

  fn get_tread_offset_left(&self) -> f64 {
    self.tread_offset_left
  }

  fn get_tread_offset_right(&self) -> f64 {
    self.tread_offset_right
  }

  fn get_turret_heading(&self) -> f64 {
    self.turret_heading
  }

  fn is_burning(&self) -> bool {
    matches!(self.state, State::Burning(_))
  }

  fn is_dry_firing(&self) -> bool {
    self.dry_firing
  }

  fn is_firing(&self) -> bool {
    self.firing
  }

  fn is_sparking(&self) -> bool {
    matches!(self.state, State::Sparking(_))
  }
}
