use crate::app::{enums::{app_state::AppState, key::Key}, graphic::run::{FIRST_TO_CHOOSE, LAST_TO_CHOOSE}, run::config::RunConfig};

pub fn update(key: Key, init: &mut Option<AppState>, config: &mut RunConfig) {
  match key {
    Key::DOWN if config.user_position.y == LAST_TO_CHOOSE => config.user_position.y = FIRST_TO_CHOOSE,
    Key::DOWN => config.user_position.y += 1,
    Key::UP if config.user_position.y == FIRST_TO_CHOOSE => config.user_position.y = LAST_TO_CHOOSE,
    Key::UP => config.user_position.y -= 1,
    Key::LEFT if config.user_position.x != 0 => config.user_position.x -= 1,
    Key::RIGHT if config.user_position.x != config.plants.len() as i8 - 1 => 
      config.user_position.x += 1,
    Key::SPACE | Key::ENTER => match config.user_position.y {
      0 => *init = Some(AppState::MENU),
      1 => *init = Some(AppState::OPTIONS),
      _ => (),
    },
    _ => (),
  }
}