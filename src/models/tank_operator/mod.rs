use super::tank::TankConsole;
use core::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct TankOperatorState {
  tank_console: Option<Rc<RefCell<dyn TankConsole>>>,
}

pub trait TankOperator {
  fn fire(&mut self);

  // TODO: was iterator
  fn get_path(&self) -> Vec<(f64, f64)>;

  fn set_tank_console(
    &mut self,
    tank_console: Rc<RefCell<dyn TankConsole>>,
  );

  fn update(
    &mut self,
    time_delta: f64,
  );
}

impl TankOperator for TankOperatorState {
  fn fire(&mut self) {
    todo!();
  }

  // TODO: was iterator
  fn get_path(&self) -> Vec<(f64, f64)> {
    todo!();
  }

  fn set_tank_console(
    &mut self,
    tank_console: Rc<RefCell<dyn TankConsole>>,
  ) {
    todo!();
  }

  fn update(
    &mut self,
    time_delta: f64,
  ) {
    todo!();
  }
}
