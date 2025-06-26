use crossterm::event::{read, Event, KeyCode};
use std::io::{stdout, Write};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};

fn clear_terminal() {
  let mut stdout = stdout();
  execute!(
    stdout,
    Clear(ClearType::All),
    MoveTo(0, 0)
  ).unwrap();
}

pub fn render(p: char) {
  clear_terminal();
  print!("{}", p)
}