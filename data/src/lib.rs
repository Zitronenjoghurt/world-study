use crate::country::{parse_countries, Country};
use crate::generic::data_map::DataMap;
use serde::{Deserialize, Serialize};

pub mod basic;
pub mod country;
mod generic;
mod traits;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldStudyData {
    pub countries: DataMap<Country>,
}

impl WorldStudyData {
    pub fn build() -> Self {
        Self {
            countries: parse_countries(),
        }
    }
}
