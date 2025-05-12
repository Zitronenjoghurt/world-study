use eframe::emath::Pos2;
use eframe::epaint::{Color32, Shape, Stroke};
use std::collections::HashMap;
use std::sync::Arc;
use world_data_types::data::country::Country;

pub fn build_country_outlines(
    countries: &HashMap<String, Arc<Country>>,
) -> HashMap<String, Vec<Shape>> {
    let mut country_outlines = HashMap::new();

    for (code, country) in countries.iter() {
        let mut shapes = Vec::new();
        for polygon in &country.polygons {
            let mut outline_points = Vec::new();
            for coord in polygon.exterior() {
                outline_points.push(Pos2::new(coord.x, -coord.y));
            }

            let shape = Shape::line(
                outline_points,
                Stroke::new(0.05, Color32::from_rgb(108, 86, 113)),
            );

            shapes.push(shape);
        }

        country_outlines.insert(code.clone(), shapes);
    }

    country_outlines
}
