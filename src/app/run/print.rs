use crate::app::run::state::State;
use crate::app::{run::config::RunConfig};
use crate::app::run::main::print::print as main_print;

pub fn print(print: &mut Vec<String>, config: &RunConfig) {
  match config.state {
    State::MAIN => main_print(print, config),
  }
}