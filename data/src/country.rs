use crate::generic::data_map::DataMap;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Country {
    pub code: String,
    pub name: String,
    pub long_name: String,
    pub region: String,
}

impl HasId for Country {
    type Id = String;

    fn id(&self) -> String {
        self.code.clone()
    }

    fn with_id(self, id: Self::Id) -> Self {
        Self { code: id, ..self }
    }
}

#[derive(Serialize, Deserialize)]
struct JsonCountry {
    pub name: String,
    pub longname: String,
    pub region: String,
}

pub fn parse_countries() -> DataMap<Country> {
    let mut map = DataMap::new();

    let file_path = PathBuf::from("data/files/countries.json");
    let file = std::fs::File::open(file_path).unwrap();
    let countries: HashMap<String, JsonCountry> = serde_json::from_reader(file).unwrap();

    for (code, data) in countries {
        let country = Country {
            code,
            name: data.name,
            long_name: data.longname,
            region: data.region,
        };
        map.add(country);
    }

    map
}
