use std::io::{self};

use crate::app::enums::key::Key;
use crate::app::event::event;
use crate::app::menu::menu::menu_update;

pub fn run() {
  let is_menu = true;
  loop {
    let e = event();
    if is_menu {
      match e {
        None => continue,
        Some(Key::Q) => return,
        Some(x) => menu_update(x),
      }
    }
  }
}