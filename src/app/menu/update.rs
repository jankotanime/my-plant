use crate::app::enums::app_state::AppState;
use crate::app::enums::key::{Key};
use crate::app::graphic::{menu::TO_CHOOSE_LEN};
use crate::app::menu::config::MenuConfig;

pub fn update(x: Key, state: &mut AppState, app: &mut bool, config: &mut MenuConfig) {
  match x {
    Key::DOWN if config.vars.stage == TO_CHOOSE_LEN - 1 => config.vars.stage = 0,
    Key::DOWN => config.vars.stage += 1,
    Key::UP if config.vars.stage == 0 => config.vars.stage = TO_CHOOSE_LEN - 1,
    Key::UP => config.vars.stage -= 1,
    Key::SPACE | Key::ENTER => match config.vars.stage {
      0 => *state = AppState::RUN,
      1 => *state = AppState::OPTIONS,
      2 => *app = false,
      _ => (),
    },
    _ => (),
  }
}