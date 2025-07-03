use crate::app::{enums::plant::Plant, graphic::{main::*, run::*}, run::config::RunConfig};

fn print_line<F: Fn(&Plant) -> String>(print: &mut Vec<String>, plants: &Vec<Plant>, line: &PositionPrint, f: F) {
  let mut to_print:String = String::new();
  for (i, plant) in plants.iter().enumerate() {
    let spaces_amount:usize = LINE_LEN as usize - (f(plant).chars().count() + line.print.chars().count());
    to_print.push_str(line.print);
    to_print.push_str(f(plant).as_str());
    to_print.push_str(" ".repeat(spaces_amount).as_str());
    if plants.len() - 1 == i {
      to_print.push_str("\n");
    } else {
      to_print.push_str(" | ")
    }
  }
  print.push(to_print);
}

pub fn print(print: &mut Vec<String>, config: &RunConfig) {
  print.push(INTRO.to_string());
  print_line(print, &config.plants, &NAME, |p| p.name.clone());
  print_line(print, &config.plants, &SPECIES, |p| p.species.clone());
  print_line(print, &config.plants, &AGE, |p| (p.age.to_string()+" dni").clone());
  print_line(print, &config.plants, &SEEDING_DATE, |p| p.seeding.to_string().clone());
  print_line(print, &config.plants, &LAST_WATER, |p| p.last_water.to_string().clone());
  print_line(print, &config.plants, &TIME_LEFT, |p| (p.time_to_dry.to_string()+" dni").clone());
  print_line(print, &config.plants, &WATER_AMOUNT, |p| (p.water_amount.to_string()+" L").clone());
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