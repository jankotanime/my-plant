use crate::app::{enums::{app_state::AppState, key::Key}, graphic::run::DOWN_PANEL_LEN, run::config::{RunConfig}};

pub fn update(key: Key, init: &mut Option<AppState>, config: &mut RunConfig) {
  match key {
    Key::DOWN if config.user_position.y == DOWN_PANEL_LEN - 1 => config.user_position.y = 0,
    Key::DOWN => config.user_position.y += 1,
    Key::UP if config.user_position.y == 0 => config.user_position.y = DOWN_PANEL_LEN - 1,
    Key::UP => config.user_position.y -= 1,
    Key::SPACE | Key::ENTER => match config.user_position.y {
      0 => *init = Some(AppState::MENU),
      1 => *init = Some(AppState::OPTIONS),
      _ => (),
    },
    _ => (),
  }
}