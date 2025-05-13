use crate::data::identified_polygon::{IdentifiedPolygon, IdentifiedPolygonType};
use crate::data::meshes::{CountryMeshes, CountryMeshesMap};
use crate::data::outlines::build_country_outlines;
use crate::data::polygon_tree::build_polygon_tree;
use eframe::emath::Vec2;
use eframe::epaint::Shape;
use egui::Image;
use geo::{Scale, SimplifyVw};
use rstar::{PointDistance, RTree, AABB};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use world_data::world_data_types::data::capital::Capital;
use world_data::world_data_types::data::country::Country;

pub mod identified_polygon;
mod meshes;
mod outlines;
mod polygon_tree;

const EXCLUDED_COUNTRY_CODES: &[&str] = &["AQ"];
const SCALED_COUNTRIES: &[(&str, f32)] = &[
    ("VA", 115.0),
    ("SM", 2.0),
    ("MC", 3.0),
    ("TV", 5.0),
    ("NR", 2.0),
];

pub struct WorldStudyData {
    countries: HashMap<String, Arc<Country>>,
    country_codes: Vec<String>,
    country_meshes: CountryMeshesMap,
    country_outlines: HashMap<String, Vec<Shape>>,
    capitals: HashMap<String, Arc<Capital>>,
    capital_names: HashSet<String>,
    polygon_tree: RTree<IdentifiedPolygon>,
}

impl WorldStudyData {
    pub fn load() -> Self {
        let scaled_countries: HashMap<&str, f32> = SCALED_COUNTRIES.iter().cloned().collect();

        let mut world_data = world_data::load();
        world_data.countries.values_mut().for_each(|country| {
            let simplified_polygons = country
                .polygons
                .iter()
                .map(|poly| {
                    let scaling_factor = scaled_countries
                        .get(country.iso_a2.as_str())
                        .copied()
                        .unwrap_or(1.0);
                    let scaled_poly = poly.scale(scaling_factor);
                    scaled_poly.simplify_vw(&0.0025)
                })
                .collect();
            country.polygons = simplified_polygons;
        });

        let countries: HashMap<String, Arc<Country>> = world_data
            .countries
            .iter()
            .map(|(code, country)| (code.clone(), Arc::new(country.clone())))
            .filter_map(|(code, country)| {
                if EXCLUDED_COUNTRY_CODES.contains(&code.as_str()) {
                    None
                } else {
                    Some((code, country))
                }
            })
            .collect();

        let mut countries_sorted: Vec<_> = countries.values().cloned().collect();
        countries_sorted.sort_by_key(|country| country.is_enclave);
        let country_codes = countries_sorted
            .iter()
            .map(|country| country.iso_a2.clone())
            .collect();
        let country_meshes = CountryMeshesMap::build(&countries);
        let country_outlines = build_country_outlines(&countries);

        let capitals = world_data
            .capitals
            .iter()
            .map(|(name, capital)| (name.to_uppercase(), Arc::new(capital.clone())))
            .collect();
        let capital_names = world_data.capitals.keys().cloned().collect();

        let polygon_tree = build_polygon_tree(&countries, &capitals);

        Self {
            countries,
            country_codes,
            country_meshes,
            country_outlines,
            capitals,
            capital_names,
            polygon_tree,
        }
    }

    pub fn get_country(&self, country_code: &str) -> Option<&Arc<Country>> {
        self.countries.get(&country_code.to_uppercase())
    }

    pub fn get_capital(&self, capital_name: &str) -> Option<&Arc<Capital>> {
        self.capitals.get(&capital_name.to_uppercase())
    }

    pub fn get_country_capitals(&self, country_code: &str) -> Vec<&Arc<Capital>> {
        let capital_names = self
            .get_country(country_code)
            .map(|country| country.capitals.clone())
            .unwrap_or_default();
        capital_names
            .iter()
            .filter_map(|name| self.get_capital(name))
            .collect()
    }

    pub fn get_countries(&self, country_codes: &[&str]) -> Vec<&Arc<Country>> {
        country_codes
            .iter()
            .filter_map(|country_code| self.get_country(country_code))
            .collect()
    }

    pub fn countries_iter(&self) -> impl Iterator<Item = &Arc<Country>> {
        self.countries.values()
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

    pub fn get_polygon_id_at_point(&self, x: f32, y: f32) -> Option<&IdentifiedPolygon> {
        let point_envelope = AABB::from_point([x, y]);
        self.polygon_tree
            .locate_in_envelope_intersecting(&point_envelope)
            .filter(|poly| poly.contains_point(&[x, y]))
            .max_by_key(|poly| {
                poly.priority() || poly.polygon_type() == IdentifiedPolygonType::Capital
            })
    }

    pub fn get_country_flag_image(&self, country_code: &str, size: Vec2) -> Option<Image> {
        self.get_country(country_code).map(|country| {
            Image::from_bytes(
                format!("bytes://flag_{}_{}x{}.svg", country.iso_a2, size.x, size.y),
                country.flag_svg.clone(),
            )
            .fit_to_exact_size(size)
        })
    }
}
