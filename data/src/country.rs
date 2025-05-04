use crate::generic::data_map::DataMap;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Type;
use svg::parser::Event;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Country {
    pub code: String,
    pub name: String,
    pub long_name: String,
    pub region: String,
    pub outlines: Vec<Vec<(f32, f32)>>,
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
    parse_svg_file();

    let mut map = DataMap::new();

    let file_path = PathBuf::from("data/files/countries.json");
    let file = std::fs::File::open(file_path).unwrap();
    let countries: HashMap<String, JsonCountry> = serde_json::from_reader(file).unwrap();
    let outlines: HashMap<String, Vec<Vec<(f32, f32)>>> = parse_svg_file();

    for (code, data) in countries {
        let country = Country {
            code: code.clone(),
            name: data.name,
            long_name: data.longname,
            region: data.region,
            outlines: outlines.get(&code).cloned().unwrap_or_default(),
        };
        map.add(country);
    }

    map
}

fn parse_svg_file() -> HashMap<String, Vec<Vec<(f32, f32)>>> {
    let file_path = PathBuf::from("data/files/world.svg");
    let content = std::fs::read_to_string(file_path).unwrap();

    let mut country_outlines = HashMap::new();

    let parser = svg::read(&content).unwrap();
    for event in parser {
        let Event::Tag("path", Type::Empty, attributes) = event else {
            continue;
        };

        let (Some(id), Some(path_data)) = (attributes.get("id"), attributes.get("d")) else {
            continue;
        };

        let lines = parse_svg_path(path_data);
        country_outlines.insert(id.to_string(), lines);
    }

    country_outlines
}

fn parse_svg_path(path_data: &str) -> Vec<Vec<(f32, f32)>> {
    let data = Data::parse(path_data).unwrap();
    let mut lines = Vec::new();

    let mut current_pos = (0.0, 0.0);
    let mut current_line = Vec::new();
    let mut is_new_line = true;

    for command in data.iter() {
        match command {
            Command::Move(_, params) => {
                for pair in params.chunks_exact(2) {
                    let (x, y) = (pair[0], pair[1]);

                    if is_new_line {
                        current_pos = (x, y);
                        is_new_line = false;
                    } else {
                        current_pos.0 += x;
                        current_pos.1 += y;
                    }

                    current_line.push(current_pos);
                }
            }
            Command::Close => {
                lines.push(current_line.clone());
                current_line.clear();
                is_new_line = true;
            }
            _ => {}
        }
    }

    lines
}
