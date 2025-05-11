use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AllCountriesEntryMaps {
    pub googleMaps: String,
    pub openStreetMaps: String,
}
