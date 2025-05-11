use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::path::PathBuf;
use world_study_data::country::JsonCountry;
use world_study_data::generic::position::Position;
use world_study_tools::models::all_countries_entry::AllCountriesEntry;
use world_study_tools::models::countries_extra::CountriesExtra;

fn main() {
    let output_path = PathBuf::from("./output");
    let result_path = output_path.join("countries.json");

    let data_path = PathBuf::from("./data");
    let all_countries_path = data_path.join("all_countries.json");
    let countries_extra_path = data_path.join("countries_extra.json");

    let extra_data: CountriesExtra =
        serde_json::from_reader(File::open(countries_extra_path).unwrap()).unwrap();
    let all_data: Vec<AllCountriesEntry> =
        serde_json::from_reader(File::open(all_countries_path).unwrap()).unwrap();

    let mut result: BTreeMap<String, JsonCountry> = BTreeMap::new();
    for entry in all_data {
        let code = entry.cca2;
        let is_enclave = extra_data.enclaves.contains(&code);
        let capitals: HashMap<String, Position> =
            extra_data.capitals.get(&code).cloned().unwrap_or_default();

        let country = JsonCountry {
            name: entry.name.common,
            official_name: entry.name.official,
            region: entry.region,
            population: entry.population,
            area: entry.area as u32,
            tlds: entry.tld,
            capitals,
            is_enclave,
        };

        result.insert(code, country);
    }

    let result_json = serde_json::to_string_pretty(&result).unwrap();
    std::fs::write(result_path, result_json).unwrap();
}
