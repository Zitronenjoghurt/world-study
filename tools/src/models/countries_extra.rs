use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use world_study_data::generic::position::Position;

#[derive(Serialize, Deserialize)]
pub struct CountriesExtra {
    pub enclaves: Vec<String>,
    pub capitals: HashMap<String, HashMap<String, Position>>,
}
