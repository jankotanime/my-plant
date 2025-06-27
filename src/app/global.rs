use crate::app::enums::app_state::AppState;

pub struct Global {
  pub state: AppState,
  pub print: String,
}

impl Global {
  pub fn new() -> Self {
    Self {
      state: AppState::MENU,
      print: String::new(),
    }
  }
}