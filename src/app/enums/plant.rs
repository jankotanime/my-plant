use chrono::prelude::*;

pub struct Plant {
  pub name: String,
  pub species: String,
  pub age: i8,
  pub seeding: NaiveDate,
  pub last_water: NaiveDate,
  pub time_to_dry: i8,
  pub water_amount: i8,
}