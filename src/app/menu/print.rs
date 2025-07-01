use crate::app::graphic::{menu::*};
use crate::app::menu::config::MenuConfig;

pub fn print(print: &mut Vec<String>, config: &MenuConfig) {
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
}