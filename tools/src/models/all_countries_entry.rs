use crate::models::all_countries_entry::maps::AllCountriesEntryMaps;
use crate::models::all_countries_entry::name::AllCountriesEntryName;
use serde::{Deserialize, Serialize};

mod maps;
mod name;

#[derive(Serialize, Deserialize)]
pub struct AllCountriesEntry {
    pub name: AllCountriesEntryName,
    #[serde(default)]
    pub tld: Vec<String>,
    pub cca2: String,
    pub ccn3: String,
    pub cca3: String,
    pub independent: bool,
    pub unMember: bool,
    #[serde(default)]
    pub capital: Vec<String>,
    pub altSpellings: Vec<String>,
    pub region: String,
    pub subregion: Option<String>,
    pub landlocked: bool,
    #[serde(default)]
    pub borders: Vec<String>,
    pub area: f64,
    pub maps: AllCountriesEntryMaps,
    pub population: u32,
    pub continents: Vec<String>,
}
