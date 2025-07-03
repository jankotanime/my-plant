pub struct PositionPrint {
  pub print: &'static str,
  pub y: i8,
}

pub const NEW_PLANT: PositionPrint = PositionPrint { print: "Dodaj roślinę\n", y: 10 };
pub const OPTIONS: PositionPrint = PositionPrint { print: "Opcje\n", y: 11 };
pub const MENU: PositionPrint = PositionPrint { print: "Powrót do menu\n", y: 12 };

pub const DOWN_PANEL: [PositionPrint; 3] = [NEW_PLANT, OPTIONS, MENU];
pub const DOWN_PANEL_LEN:i8 = DOWN_PANEL.len() as i8;

pub const NAME: PositionPrint = PositionPrint { print: "Roślina: ", y: 0 };
pub const SPECIES: PositionPrint = PositionPrint { print: "Gatunek: ", y: 1 };
pub const AGE: PositionPrint = PositionPrint { print: "Wiek: ", y: 2 };
pub const SEEDING_DATE: PositionPrint = PositionPrint { print: "Data wysiewu: ", y: 3 };
pub const LAST_WATER: PositionPrint = PositionPrint { print: "Ostatnio podlany: ", y: 4 };
pub const TIME_LEFT: PositionPrint = PositionPrint { print: "Pozostały czas do uschnięcia: ", y: 5 };
pub const WATER_AMOUNT: PositionPrint = PositionPrint { print: "Ilość wody: ", y: 6 };
pub const WATER: PositionPrint = PositionPrint { print: "Podlej", y: 7 };
pub const CHANGE_NAME: PositionPrint = PositionPrint { print: "Zmień nazwę", y: 8 };
pub const DELETE: PositionPrint = PositionPrint { print: "Usuń", y: 9 };
pub const IMAGE: PositionPrint = PositionPrint { print: "Zmień obraz", y: 10 };

pub const PLANT_PANEL: [PositionPrint; 4] = [WATER, CHANGE_NAME, DELETE, IMAGE];
pub const PLANT_PANEL_LEN:i8 = DOWN_PANEL.len() as i8;

pub const FIRST_TO_CHOOSE:i8 = 6;
pub const LAST_TO_CHOOSE:i8 = 12;