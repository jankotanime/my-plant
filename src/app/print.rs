use crate::app::{enums::app_state::AppState, global::{AppConfig, Global}};
use crate::app::menu::print::print  as menu_print;

pub fn print(global: &mut Global) {
  match global.state {
    AppState::MENU => 
      if let AppConfig::Menu(menu_config) = &global.config {
        menu_print(&mut global.print, menu_config);
      },
    AppState::RUN => (),
    _ => return,
  }
}