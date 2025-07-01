use crate::app::{enums::point::Point, graphic::run::FIRST_TO_CHOOSE, run::state::State};

pub struct RunConfig {
  pub user_position: Point,
  pub state: State,
  pub down_panel: bool,
}

impl RunConfig {
  pub fn new() -> Self {
    Self {
      user_position: Point { x: 0, y: FIRST_TO_CHOOSE },
      state: State::MAIN,
      down_panel: true,
    }
  }
}