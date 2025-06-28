use std::io::{stdout, Write};
use crossterm::{execute, terminal::Clear, terminal::ClearType, cursor::MoveTo};

fn clear_terminal(stdout: &mut impl Write) {
  execute!(
    stdout,
    Clear(ClearType::All),
    MoveTo(0, 0)
  ).unwrap();
}

pub fn render(print: &mut String) {
  let mut stdout = stdout();
  clear_terminal(&mut stdout);
  write!(stdout, "{}", print).unwrap();
  stdout.flush().unwrap();
  *print = "".to_string();
}
