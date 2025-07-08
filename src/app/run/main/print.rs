use crate::app::{structs::plant::Plant, graphic::{main::*, run::*}, run::config::RunConfig};

fn print_line<F: Fn(&Plant) -> String>(print: &mut Vec<String>, plants: &Vec<Plant>, x: &i8, line: &PositionPrint, f: F) {
  let mut to_print:String = String::new();
  for (i, plant) in plants.iter().enumerate() {
    if 0 <= (i as i8 - x) && (i as i8 - x) <= 2 {
      let spaces_amount:usize = LINE_LEN as usize - (f(plant).chars().count() + line.print.chars().count());
      to_print.push_str(line.print);
      to_print.push_str(f(plant).as_str());
      to_print.push_str(" ".repeat(spaces_amount).as_str());
      to_print.push_str(" | ")
    }
  }
  print.push(to_print);
}

fn print_plant_panel(print: &mut Vec<String>, plants: &Vec<Plant>, x: &i8, y: &i8, line_number: i8, line: &PositionPrint) {
  let mut to_print:String = String::new();
  for i in 0..plants.len() {
    if 0 <= (i as i8 - x) && (i as i8 - x) <= 2 {
      let spaces_amount:usize = LINE_LEN as usize - (line.print.chars().count() + LINE_STARTER_LEN as usize);
      if line_number + FIRST_TO_CHOOSE == *y && i as i8 - x == 0 {
        to_print.push_str(CHOOSE);
      } else {
        to_print.push_str(NOT_CHOOSE);
      }
      to_print.push_str(line.print);
      to_print.push_str(" ".repeat(spaces_amount).as_str());
      to_print.push_str(" | ")
    }
  }
  print.push(to_print);
}

pub fn print(print: &mut Vec<String>, config: &RunConfig) {
  print.push(INTRO.to_string());
  print_line(print, &config.plants, &config.user_position.x, &NAME, |p| p.name.clone());
  print_line(print, &config.plants, &config.user_position.x, &SPECIES, |p| p.species.clone());
  print_line(print, &config.plants, &config.user_position.x, &AGE, |p| (p.age.to_string()+" dni").clone());
  print_line(print, &config.plants, &config.user_position.x, &SEEDING_DATE, |p| p.seeding.to_string().clone());
  print_line(print, &config.plants, &config.user_position.x, &LAST_WATER, |p| p.last_water.to_string().clone());
  print_line(print, &config.plants, &config.user_position.x, &TIME_LEFT, |p| (p.time_to_dry.to_string()+" dni").clone());
  print_line(print, &config.plants, &config.user_position.x, &WATER_AMOUNT, |p| (p.water_amount.to_string()+" L").clone());
  print_line(print, &config.plants, &config.user_position.x, &IS_ALIVE, |p| (p.alive.to_string()).clone());

  for (i, line) in PLANT_PANEL.iter().enumerate() {
    print_plant_panel(print, &config.plants, &config.user_position.x, &config.user_position.y, i as i8, line);
  }

  print.push("".to_string());

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