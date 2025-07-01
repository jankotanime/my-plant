use crate::app::enums::key::Key;
use crate::app::event::event;
use crate::app::global::{Global, AppConfig};
use crate::app::menu::print::print  as menu_print;
use crate::app::print::print;
use crate::app::render::render;
use crate::app::update::update;

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
    print(&mut global);
    render(&mut global.print);
  }
}