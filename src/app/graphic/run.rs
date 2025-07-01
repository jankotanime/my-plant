use crate::app::enums::point::Point;

pub struct PositionPrint {
  pub print: &'static str,
  pub position: &'static Point,
}

pub const MENU: PositionPrint = PositionPrint { print: "Powrót do menu\n", position: &Point { x: 0, y: 0 } };
pub const OPTIONS: PositionPrint = PositionPrint { print: "Opcje\n", position: &Point { x: 0, y: 0 } };

pub const DOWN_PANEL: [PositionPrint; 2] = [MENU, OPTIONS];
pub const DOWN_PANEL_LEN:i8 = DOWN_PANEL.len() as i8;