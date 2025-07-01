use crate::app::enums::key::Key;
use crate::app::menu::print::print  as menu_print;
use crate::app::main::event::event;
use crate::app::main::global::{Global, AppConfig};
use crate::app::main::initialize::initialize;
use crate::app::main::print::print;
use crate::app::main::render::render;
use crate::app::main::update::update;

pub fn run() {
  let mut global = Global::new();
  if let AppConfig::Menu(menu_config) = &global.config {
    menu_print(&mut global.print, menu_config);
  }
  render(&mut global.print);
  while global.app {
    let event = event();
    match event {
      None => continue,
      Some(Key::Q) => return,
      Some(key) => update(&mut global, key)
    }
    match global.init {
      None => (),
      Some(init) => {
        initialize(init, &mut global.state, &mut global.config);
        global.init = None
      },
    }
    print(&mut global);
    render(&mut global.print);
  }
}