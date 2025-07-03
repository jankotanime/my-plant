use crate::app::{graphic::{main::*, run::*}, run::config::RunConfig};

pub fn print(print: &mut Vec<String>, config: &RunConfig) {
  print.push(INTRO.to_string());
  for (i, plant) in config.plants.iter().enumerate() {
    let spaces_amount:usize = (LINE_LEN - LINE_STARTER_LEN) as usize - NAME.print.len();
    print.push(format!("{}{}{}{}{}", NOT_CHOOSE, NAME.print, plant.name, " ".repeat(spaces_amount), if i == config.plants.len() - 1 {"\n"} else {"|"}));
    let spaces_amount:usize = (LINE_LEN - LINE_STARTER_LEN) as usize - NAME.print.len();
    print.push(format!("{}{}{}{}{}", NOT_CHOOSE, SPECIES.print, plant.species, " ".repeat(spaces_amount), if i == config.plants.len() - 1 {"\n"} else {"|"}));
    let spaces_amount:usize = (LINE_LEN - LINE_STARTER_LEN) as usize - NAME.print.len() - 4;
    print.push(format!("{}{}{} dni{}{}", NOT_CHOOSE, AGE.print, plant.age, " ".repeat(spaces_amount), if i == config.plants.len() - 1 {"\n"} else {"|"}));
    let spaces_amount:usize = (LINE_LEN - LINE_STARTER_LEN) as usize - NAME.print.len();
    print.push(format!("{}{}{}{}{}", NOT_CHOOSE, SEEDING_DATE.print, plant.seeding, " ".repeat(spaces_amount), if i == config.plants.len() - 1 {"\n"} else {"|"}));
    let spaces_amount:usize = (LINE_LEN - LINE_STARTER_LEN) as usize - NAME.print.len();
    print.push(format!("{}{}{}{}{}", NOT_CHOOSE, LAST_WATER.print, plant.last_water, " ".repeat(spaces_amount), if i == config.plants.len() - 1 {"\n"} else {"|"}));
    let spaces_amount:usize = (LINE_LEN - LINE_STARTER_LEN) as usize - NAME.print.len() - 4;
    print.push(format!("{}{}{} dni{}{}", NOT_CHOOSE, TIME_LEFT.print, plant.time_to_dry, " ".repeat(spaces_amount), if i == config.plants.len() - 1 {"\n"} else {"|"}));
    let spaces_amount:usize = (LINE_LEN - LINE_STARTER_LEN) as usize - NAME.print.len() - 2;
    print.push(format!("{}{}{} L{}{}", NOT_CHOOSE, WATER_AMOUNT.print, plant.water_amount, " ".repeat(spaces_amount), if i == config.plants.len() - 1 {"\n"} else {"|"}));
    for i in 0..PLANT_PANEL_LEN {
      let mut to_print = String::new();
      if PLANT_PANEL[i as usize].y == config.user_position.y {
        to_print.push_str(CHOOSE);
      } else {
        to_print.push_str(NOT_CHOOSE);
      }
      to_print.push_str(PLANT_PANEL[i as usize].print);
      print.push(to_print);
    }
  }
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