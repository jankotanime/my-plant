use crate::app::{enums::{app_state::AppState, key::Key}, global::{AppConfig, Global}};
use crate::app::menu::update::update as menu_update;
// use crate::app::run::update::update as run_update;


pub fn update(global: &mut Global, key: Key) {
  match global.state {
    AppState::MENU => 
      if let AppConfig::Menu(menu_config) = &mut global.config {
        menu_update(key, &mut global.state, &mut global.app, &mut **menu_config);
      },
    AppState::RUN => (),
    _ => return,
  }
}