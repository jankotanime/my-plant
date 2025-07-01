use crate::app::{enums::app_state::AppState, main::global::AppConfig, menu::config::MenuConfig, run::{config::RunConfig}};
use crate::app::run::init::init as run_init;

pub fn initialize(init: AppState, state: &mut AppState, config: &mut AppConfig) {
  match init {
    AppState::MENU => *config = AppConfig::Menu(Box::new(MenuConfig::new())),
    AppState::RUN => {
      *config = AppConfig::Run(Box::new(RunConfig::new()));
      if let AppConfig::Run(run_config) = config {
        run_init(run_config);
      }
    }
    _ => {}
  }
  *state = init;
}