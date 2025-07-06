use crate::app::{enums::{plant::Plant, plant_record::PlantRecord}, run::config::RunConfig};
use chrono::prelude::*;
use csv::Reader;
use std::{error::Error, process, str::FromStr};


fn read_csv(config: &mut RunConfig) -> Result<(), Box<dyn Error>> {
  let today = Local::now().date_naive();
  let mut rdr = Reader::from_path("src/data/plants.csv")?;

  for result in rdr.deserialize() {
      let record: PlantRecord = result?;
      let water_consumption_speed = 1;
      let string_seeding_date: Vec<&str> = record.seeding.split('-').collect();
      println!("{}", string_seeding_date[0]);
      let seeding_date: NaiveDate = NaiveDate::from_ymd_opt(
        FromStr::from_str(string_seeding_date[0]).unwrap(),
        FromStr::from_str(string_seeding_date[1]).unwrap(),
        FromStr::from_str(string_seeding_date[2]).unwrap() 
      ).unwrap();
      let string_last_watering: Vec<&str> = record.last_watering.split('-').collect();
      let last_watering: NaiveDate = NaiveDate::from_ymd_opt(
        FromStr::from_str(string_last_watering[0]).unwrap(),
        FromStr::from_str(string_last_watering[1]).unwrap(),
        FromStr::from_str(string_last_watering[2]).unwrap() 
      ).unwrap();
      let now_water = record.water - (today - last_watering).num_days()*water_consumption_speed;
      config.plants.push(Plant { 
        name: record.name,
        species: record.species, age: (today - last_watering).num_days(), 
        seeding: seeding_date, last_water: last_watering, 
        time_to_dry: now_water*water_consumption_speed, 
        water_amount: now_water, alive: record.alive });
  }
  Ok(())
}

pub fn init(config: &mut RunConfig) {
  if let Err(err) = read_csv(config) {
    println!("{}", err);
    process::exit(1);
  }
}