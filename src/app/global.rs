use crate::app::enums::app_state::AppState;
use crate::app::menu::config::MenuConfig;
use crate::app::run::config::RunConfig;

pub struct Global {
  pub state: AppState,
  pub print: Vec<String>,
  pub config: AppConfig,
  pub app: bool,
}

pub enum AppConfig {
  Menu(Box<MenuConfig>),
  Run(Box<RunConfig>),
}

impl Global {
  pub fn new() -> Self {
    Self {
      state: AppState::MENU,
      print: Vec::new(),
      config: AppConfig::Menu(Box::new(MenuConfig::new())),
      app: true,
    }
  }
}
