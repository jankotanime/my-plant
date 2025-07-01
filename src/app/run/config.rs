use crate::app::enums::point::Point;

pub struct RunConfig {
  pub user_position: Point,
}

impl RunConfig {
  pub fn new() -> Self {
    Self {
      user_position: Point { x: 0, y: 0 },
    }
  }
}