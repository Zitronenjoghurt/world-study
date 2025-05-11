use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AllCountriesEntryName {
    pub common: String,
    pub official: String,
}
