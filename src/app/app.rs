use crate::app::enums::app_state::AppState;
use crate::app::enums::key::Key;
use crate::app::event::event;
use crate::app::global::Global;
use crate::app::menu::config::Config;
use crate::app::menu::update::update as menu_update;
use crate::app::menu::print::print  as menu_print;
use crate::app::run::update::update as run_update;
use crate::app::render::render;

pub fn run() {
  let mut global = Global::new();
  let mut config = Config::new();
  menu_print(&mut global.print, &mut config);
  render(&mut global.print);
  loop {
    let event = event();
    match event {
      None => continue,
      Some(Key::Q) => return,
      Some(key) => {
        match global.state {
          AppState::MENU => match key {
            Key::SPACE | Key::ENTER if config.vars.stage == 2 => return,
            _ => menu_update(key, &mut global.state, &mut config)
          },
          AppState::RUN => (),
          _ => return,
        }
      }
    }
    match global.state {
      AppState::MENU => menu_print(&mut global.print, &mut config),
      AppState::RUN => (),
      _ => return,
    }
    render(&mut global.print);
  }
}