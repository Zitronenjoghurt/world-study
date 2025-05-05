use crate::country::{parse_countries, Country};
use crate::generic::data_map::DataMap;
use egui::{Color32, Pos2, Shape, Stroke};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub mod country;
mod generic;
mod traits;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldStudyData {
    countries: DataMap<Country>,
    #[serde(skip, default)]
    countries_by_region: HashMap<String, HashSet<String>>,
    #[serde(skip, default)]
    country_codes: HashSet<String>,
    #[serde(skip, default)]
    regions: HashSet<String>,
    #[serde(skip, default)]
    country_outlines: HashMap<String, Vec<Shape>>,
    #[serde(skip, default)]
    country_meshes: HashMap<String, Vec<Shape>>,
}

impl WorldStudyData {
    pub fn build() -> Self {
        let countries = parse_countries();
        Self {
            countries,
            countries_by_region: HashMap::new(),
            regions: HashSet::new(),
            country_codes: HashSet::new(),
            country_outlines: HashMap::new(),
            country_meshes: HashMap::new(),
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

        self.country_outlines = initialize_country_outlines(&self.countries);
        self.country_meshes = initialize_country_meshes(&self.countries);
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

    pub fn get_country_outlines(&self, country_code: &str) -> Option<&Vec<Shape>> {
        self.country_outlines.get(country_code)
    }

    pub fn get_country_meshes(&self, country_code: &str) -> Option<&Vec<Shape>> {
        self.country_meshes.get(country_code)
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

fn initialize_country_outlines(countries: &DataMap<Country>) -> HashMap<String, Vec<Shape>> {
    let mut country_outlines = HashMap::new();
    for country in countries.iter() {
        let mut shapes = Vec::new();
        for line in &country.outlines {
            let mut outline_points = Vec::new();
            for (x, y) in line {
                outline_points.push(Pos2::new(*x, *y));
            }
            let shape = Shape::line(
                outline_points,
                Stroke::new(0.2, Color32::from_rgb(255, 255, 255)),
            );
            shapes.push(shape);
        }
        country_outlines.insert(country.code.clone(), shapes);
    }
    country_outlines
}

fn initialize_country_meshes(countries: &DataMap<Country>) -> HashMap<String, Vec<Shape>> {
    let mut country_meshes = HashMap::new();
    for country in countries.iter() {
        let mut meshes = Vec::new();

        for line in &country.outlines {
            let mut outline_points = Vec::new();
            let mut flat_points = Vec::new();
            for (x, y) in line {
                outline_points.push(Pos2::new(*x, *y));
                flat_points.push(*x as f64);
                flat_points.push(*y as f64);
            }

            let mut mesh = egui::Mesh::default();
            mesh.vertices.reserve(outline_points.len());

            if let Ok(indices) = earcutr::earcut(&flat_points, &[], 2) {
                let mut mesh = egui::Mesh::default();
                mesh.vertices.reserve(outline_points.len());

                for point in &outline_points {
                    mesh.vertices.push(egui::epaint::Vertex {
                        pos: *point,
                        uv: Pos2::ZERO,
                        color: Color32::from_rgba_premultiplied(60, 60, 180, 40),
                    });
                }

                for chunk in indices.chunks(3) {
                    if chunk.len() == 3 {
                        mesh.indices.push(chunk[0] as u32);
                        mesh.indices.push(chunk[1] as u32);
                        mesh.indices.push(chunk[2] as u32);
                    }
                }

                meshes.push(Shape::Mesh(Arc::from(mesh)));
            }
        }

        country_meshes.insert(country.code.clone(), meshes);
    }

    country_meshes
}
