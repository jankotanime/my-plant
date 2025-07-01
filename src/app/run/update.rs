use crate::app::{enums::{app_state::AppState, key::Key}, run::{config::RunConfig, state::State}};
use crate::app::run::main::update::update as main_update;

pub fn update(key: Key, init: &mut Option<AppState>, config: &mut RunConfig) {
  match config.state {
    State::MAIN => main_update(key, init, config),
  }
}