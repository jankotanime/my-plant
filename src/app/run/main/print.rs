use crate::app::{graphic::run::*, graphic::main::*, run::config::RunConfig};

pub fn print(print: &mut Vec<String>, config: &RunConfig) {
  print.push(INTRO.to_string());
  for i in 0..DOWN_PANEL_LEN {
    let mut to_print = String::new();
    if DOWN_PANEL[i as usize].y == config.user_position.y {
      to_print.push_str(CHOOSE);
    } else {
      to_print.push_str(NOT_CHOOSE);
    }
    to_print.push_str(DOWN_PANEL[i as usize].print);
    print.push(to_print);
  }
}