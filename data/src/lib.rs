use crate::country::{parse_countries, Country};
use crate::generic::data_map::DataMap;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub mod country;
mod generic;
mod traits;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldStudyData {
    countries: DataMap<Country>,
    countries_by_region: HashMap<String, HashSet<String>>,
    country_codes: HashSet<String>,
    regions: HashSet<String>,
}

impl WorldStudyData {
    pub fn build() -> Self {
        let countries = parse_countries();
        Self {
            countries,
            countries_by_region: HashMap::new(),
            regions: HashSet::new(),
            country_codes: HashSet::new(),
        }
    }

    pub fn initialize(&mut self) {
        let mut countries_by_region: HashMap<String, HashSet<String>> = HashMap::new();
        for country in self.countries.iter() {
            countries_by_region
                .entry(country.region.clone())
                .or_default()
                .insert(country.code.clone());
        }
        self.countries_by_region = countries_by_region;
        self.regions = self.countries_by_region.keys().cloned().collect();
        self.country_codes = self.countries.keys().cloned().collect();
    }

    pub fn get_country(&self, country_code: &str) -> Option<&Arc<Country>> {
        self.countries.get(&country_code.to_uppercase())
    }

    pub fn get_countries(&self, country_codes: &[&str]) -> Vec<&Arc<Country>> {
        country_codes
            .iter()
            .filter_map(|country_code| self.get_country(country_code))
            .collect()
    }

    pub fn countries_iter(&self) -> impl Iterator<Item = &Arc<Country>> {
        self.countries.iter()
    }

    pub fn get_country_codes(&self) -> &HashSet<String> {
        &self.country_codes
    }

    pub fn get_regions(&self) -> &HashSet<String> {
        &self.regions
    }

    pub fn country_exists(&self, country_code: &str) -> bool {
        self.country_codes.contains(&country_code.to_uppercase())
    }

    pub fn region_exists(&self, region: &str) -> bool {
        self.regions.contains(&region.to_uppercase())
    }
}
