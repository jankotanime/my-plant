use crate::app::{enums::app_state::AppState, main::global::{AppConfig, Global}};
use crate::app::menu::print::print  as menu_print;
use crate::app::run::print::print  as run_print;

pub fn print(global: &mut Global) {
  match global.state {
    AppState::MENU => 
      if let AppConfig::Menu(menu_config) = &global.config {
        menu_print(&mut global.print, menu_config);
      },
    AppState::RUN => 
    if let AppConfig::Run(run_config) = &global.config {
      run_print(&mut global.print, run_config);
    },
    _ => return,
  }
}