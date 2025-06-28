use crate::app::enums::app_state::AppState;
use crate::app::enums::key::Key;
use crate::app::event::event;
use crate::app::global::Global;
use crate::app::menu::config::Config;
use crate::app::menu::menu::menu_update;
use crate::app::render::render;

pub fn run() {
  let mut global = Global::new();
  let mut config = Config::new();
  loop {
    let e = event();
    match e {
      None => continue,
      Some(Key::Q) => return,
      Some(x) => {
        match global.state {
          AppState::MENU => menu_update(x, &mut global.print, &mut config),
          _ => continue,
        }
      }
    }
    render(&mut global.print);
  }
}