use crate::app::enums::key::Key;
use crate::app::graphic::{menu::*};
use crate::app::menu::config::Config;

pub fn menu_update(x: Key, print: &mut Vec<String>, config: &mut Config) {
  print.push(INTRO.to_string());
  for i in 0..TO_CHOOSE_LEN {
    let mut to_print = String::new();
    if i == config.vars.stage {
      to_print.push_str(CHOOSE);
    } else {
      to_print.push_str(NOT_CHOOSE);
    }
    to_print.push_str(TO_CHOOSE[i as usize]);
    print.push(to_print);
  }
  match x {
    Key::DOWN if config.vars.stage == 1 => config.vars.stage = 0,
    Key::DOWN => config.vars.stage += 1,
    Key::UP if config.vars.stage == 0 => config.vars.stage = TO_CHOOSE_LEN - 1,
    Key::UP => config.vars.stage -= 1,
    _ => (),
  }
}