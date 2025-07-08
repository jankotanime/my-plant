use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Species {
    pub name: String,
    pub max_time_without_water: i64,
    pub max_water: i64,
    pub consumption_speed: i64,
}
