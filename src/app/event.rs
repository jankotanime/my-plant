use crossterm::{event::{read, Event, KeyCode}};

use crate::app::enums::key::Key;

pub fn event()-> Option<Key> {
  if let Event::Key(key_event) = read().unwrap() {
    match key_event.code {
      KeyCode::Char('q') => {
        return Some(Key::Q);
      }
      KeyCode::Left => {
        return Some(Key::LEFT);
      }
      KeyCode::Right => {
        return Some(Key::RIGHT);
      }
      KeyCode::Up => {
        return Some(Key::UP);
      }
      KeyCode::Down => {
        return Some(Key::DOWN);
      }
      KeyCode::Char(' ') => {
        return Some(Key::SPACE);
      }
      KeyCode::Enter => {
        return Some(Key::ENTER);
      }
      _ => {
        return None;
      }
    }
  }
  return None
}