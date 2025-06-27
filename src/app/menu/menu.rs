use crate::app::enums::key::Key;

pub fn menu_update(x: Key, print: &mut String) {
  match x {
    Key::DOWN => *print = "W dół".to_string(),
    Key::LEFT => *print = "W lewo".to_string(),
    Key::RIGHT => *print = "W prawo".to_string(),
    Key::UP => *print = "W górę".to_string(),
    _ => (),
  }
}