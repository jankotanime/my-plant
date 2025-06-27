use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

fn main() {
  enable_raw_mode().expect("Nie można włączyć trybu raw");
  my_plant::app::app::run();
  println!("\nWyłączanie aplikacji...");
  disable_raw_mode().expect("Nie można wyłączyć trybu raw");
}
