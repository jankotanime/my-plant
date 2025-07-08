use chrono::prelude::*;

pub struct Plant {
  pub name: String,
  pub species: String,
  pub age: i64,
  pub seeding: NaiveDate,
  pub last_water: NaiveDate,
  pub time_to_dry: i64,
  pub water_amount: i64,
  pub alive: bool,
}