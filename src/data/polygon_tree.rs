use crate::data::identified_polygon::{IdentifiedPolygon, IdentifiedPolygonType};
use geo::{coord, Coord, LineString, Polygon};
use rstar::RTree;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::Arc;
use world_data::world_data_types::data::capital::Capital;
use world_data::world_data_types::data::country::Country;

pub fn build_polygon_tree(
    countries: &HashMap<String, Arc<Country>>,
    capitals: &HashMap<String, Arc<Capital>>,
) -> RTree<IdentifiedPolygon> {
    let mut polygons = Vec::new();

    for (code, country) in countries.iter() {
        for polygon in &country.polygons {
            let id_polygon = IdentifiedPolygon::new(
                code.clone(),
                polygon.clone(),
                country.is_enclave,
                IdentifiedPolygonType::Country,
            );
            polygons.push(id_polygon);
        }
    }

    for (name, capital) in capitals.iter() {
        let polygon = octagon_from_center(capital.coordinates, 0.1);
        let id_polygon =
            IdentifiedPolygon::new(name.clone(), polygon, false, IdentifiedPolygonType::Capital);
        polygons.push(id_polygon);
    }

    RTree::bulk_load(polygons)
}

fn octagon_from_center(center: Coord<f32>, radius: f32) -> Polygon<f32> {
    let mut points = Vec::with_capacity(9);
    for i in 0..8 {
        let angle = (i as f32) * (2.0 * PI / 8.0);
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        points.push(coord! { x: x, y: y });
    }
    points.push(points[0]);
    Polygon::new(LineString::from(points), vec![])
}
