use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlantRecord {
    pub name: String,
    pub species: String,
    pub seeding: String,
    pub water: i64,
    pub last_watering: String,
    pub alive: bool,
}
