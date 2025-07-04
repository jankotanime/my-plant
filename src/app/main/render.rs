use std::io::{stdout, Write};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::process::Command;

fn clear_terminal(stdout: &mut impl Write) {
  Command::new("clear")
    .status()
    .unwrap();
  execute!(
    stdout,
    Clear(ClearType::All),
    MoveTo(0, 0)
  ).unwrap();
}

pub fn render(print: &mut Vec<String>) {
  let mut stdout = stdout();
  clear_terminal(&mut stdout);
  for (i, line) in print.iter().enumerate() {
    execute!(stdout, MoveTo(0, i as u16)).unwrap();
    write!(stdout, "{}", line).unwrap();
  }
  stdout.flush().unwrap();
  *print = Vec::new();
}
