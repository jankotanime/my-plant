use crate::app::{enums::{app_state::AppState, key::Key}, main::global::{AppConfig, Global}};
use crate::app::menu::update::update as menu_update;
use crate::app::run::update::update as run_update;


pub fn update(global: &mut Global, key: Key) {
  match global.state {
    AppState::MENU => 
      if let AppConfig::Menu(menu_config) = &mut global.config {
        menu_update(key, &mut global.init, &mut global.app, &mut **menu_config);
      },
    AppState::RUN => 
      if let AppConfig::Run(run_config) = &mut global.config {
        run_update(key, &mut global.init, &mut **run_config);
      },
    _ => return,
  }
}