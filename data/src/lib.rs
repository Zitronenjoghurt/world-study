use crate::country::meshes::{CountryMeshes, CountryMeshesMap};
use crate::country::{parse_countries, Country};
use crate::generic::data_map::DataMap;
use crate::generic::identified_polygon::IdentifiedPolygon;
use crate::generic::position::Position;
use egui::{Color32, Image, Pos2, Shape, Stroke, Vec2};
use geo::{Coord, LineString, Polygon};
use rstar::{PointDistance, RTree, AABB};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub mod country;
pub mod generic;
mod traits;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldStudyData {
    countries: DataMap<Country>,
    #[serde(skip, default)]
    countries_by_region: HashMap<String, HashSet<String>>,
    #[serde(skip, default)]
    country_codes: Vec<String>,
    #[serde(skip, default)]
    regions: HashSet<String>,
    #[serde(skip, default)]
    country_outlines: HashMap<String, Vec<Shape>>,
    #[serde(skip, default)]
    country_meshes: CountryMeshesMap,
    #[serde(skip, default)]
    country_polygon_tree: RTree<IdentifiedPolygon>,
}

impl WorldStudyData {
    pub fn build() -> Self {
        let countries = parse_countries();
        Self {
            countries,
            countries_by_region: HashMap::new(),
            regions: HashSet::new(),
            country_codes: Vec::new(),
            country_outlines: HashMap::new(),
            country_meshes: CountryMeshesMap::default(),
            country_polygon_tree: RTree::new(),
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

        // This will ensure that enclaves are always at the end of the list
        let mut country_vec: Vec<_> = self.countries.iter().collect();
        country_vec.sort_by_key(|country| country.is_enclave);
        self.country_codes = country_vec
            .into_iter()
            .map(|country| country.code.clone())
            .collect();

        self.regions = self.countries_by_region.keys().cloned().collect();
        self.country_outlines = initialize_country_outlines(&self.countries);
        self.country_meshes = CountryMeshesMap::build(&self.countries);
        self.country_polygon_tree = initialize_country_polygon_tree(&self.countries);
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

    pub fn get_country_codes(&self) -> &Vec<String> {
        &self.country_codes
    }

    pub fn get_country_outlines(&self, country_code: &str) -> Option<&Vec<Shape>> {
        self.country_outlines.get(country_code)
    }

    pub fn get_country_meshes(&self, country_code: &str) -> Option<&CountryMeshes> {
        self.country_meshes.get(country_code)
    }

    pub fn get_country_capitals(&self, country_code: &str) -> Option<&HashMap<String, Position>> {
        self.countries
            .get(country_code)
            .map(|country| &country.capitals)
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

    pub fn get_country_code_at_point(&self, x: f32, y: f32) -> Option<String> {
        let point_envelope = AABB::from_point([x as f64, y as f64]);
        self.country_polygon_tree
            .locate_in_envelope_intersecting(&point_envelope)
            .filter(|poly| poly.contains_point(&[x as f64, y as f64]))
            .max_by_key(|poly| poly.priority())
            .map(|poly| poly.id().to_owned())
    }

    pub fn get_country_flag_image(&self, country_code: &str, size: Vec2) -> Option<Image> {
        self.get_country(country_code).map(|country| {
            Image::from_bytes(
                format!("bytes://flag_{}_{}x{}.svg", country.code, size.x, size.y),
                country.flag_svg.clone(),
            )
            .fit_to_exact_size(size)
        })
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
                Stroke::new(0.1, Color32::from_rgb(108, 86, 113)),
            );

            shapes.push(shape);
        }

        country_outlines.insert(country.code.clone(), shapes);
    }

    country_outlines
}

fn initialize_country_polygon_tree(countries: &DataMap<Country>) -> RTree<IdentifiedPolygon> {
    let mut polygons = Vec::new();

    for country in countries.iter() {
        for outline in &country.outlines {
            let mut points = Vec::new();
            for (x, y) in outline {
                points.push(Coord::from((*x as f64, *y as f64)));
            }

            let exterior = LineString(points);
            let polygon = Polygon::new(exterior, vec![]);
            let id_polygon =
                IdentifiedPolygon::new(country.code.clone(), polygon, country.is_enclave);
            polygons.push(id_polygon);
        }
    }

    RTree::bulk_load(polygons)
}

fn initialize_country_flag_images(countries: &DataMap<Country>) -> HashMap<String, Image> {
    let mut flag_images = HashMap::new();

    for country in countries.iter() {
        let image = Image::from_bytes(
            format!("bytes://flag_{}.svg", country.code),
            country.flag_svg.clone(),
        );
        flag_images.insert(country.code.clone(), image);
    }

    flag_images
}
