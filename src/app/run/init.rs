use crate::app::{enums::plant::Plant, run::config::RunConfig};
use chrono::prelude::*;

pub fn init(config: &mut RunConfig) {
  // let today = Local::now().date_naive();

  let name = "Test";
  let spec = "Test";
  let sd = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap();
  let lw = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap();
  let tl = 0;
  let wa = 1;
  let age = 12;

  let p = Plant{
    name: name.to_string(), 
    species: spec.to_string(), 
    age: age, 
    seeding: sd, 
    last_water: lw, 
    time_to_dry: tl, 
    water_amount: wa
  };

  config.plants.push(p);

  let name = "Test2";
  let spec = "Test2";
  let sd = NaiveDate::from_ymd_opt(2015, 7, 8).unwrap();
  let lw = NaiveDate::from_ymd_opt(2015, 7, 8).unwrap();
  let tl = 1;
  let wa = 0;
  let age = 10;

  let q = Plant{
    name: name.to_string(), 
    species: spec.to_string(), 
    age: age, 
    seeding: sd, 
    last_water: lw, 
    time_to_dry: tl, 
    water_amount: wa
  };

  config.plants.push(q);
}