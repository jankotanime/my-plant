use crate::app::{run::config::RunConfig, structs::{plant::Plant, plant_record::PlantRecord, species::Species}};
use chrono::prelude::*;
use csv::Reader;
use std::{error::Error, process, str::FromStr};


fn read_csv(config: &mut RunConfig) -> Result<(), Box<dyn Error>> {
  let mut rdr = Reader::from_path("src/data/species.csv")?;
  let mut all_species: Vec<Species> = Vec::new();

  for result in rdr.deserialize() {
    all_species.push(result?);
  }

  let today = Local::now().date_naive();
  rdr = Reader::from_path("src/data/plants.csv")?;

  for result in rdr.deserialize() {
    let record: PlantRecord = result?;
    let option_species = all_species.iter().find(
      |&s| s.name == record.species
    );
    match option_species {
      Some(species) => {
        let water_consumption_speed = species.consumption_speed;
        let string_seeding_date: Vec<&str> = record.seeding.split('-').collect();
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
      _ => ()
    }
  }
  Ok(())
}

pub fn init(config: &mut RunConfig) {
  if let Err(err) = read_csv(config) {
    println!("{}", err);
    process::exit(1);
  }
}