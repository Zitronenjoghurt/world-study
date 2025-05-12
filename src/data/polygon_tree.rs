use crate::data::identified_polygon::IdentifiedPolygon;
use rstar::RTree;
use std::collections::HashMap;
use std::sync::Arc;
use world_data_types::data::country::Country;

pub fn build_country_polygon_tree(
    countries: &HashMap<String, Arc<Country>>,
) -> RTree<IdentifiedPolygon> {
    let mut polygons = Vec::new();

    for (code, country) in countries.iter() {
        for polygon in &country.polygons {
            let id_polygon =
                IdentifiedPolygon::new(code.clone(), polygon.clone(), country.is_enclave);
            polygons.push(id_polygon);
        }
    }

    RTree::bulk_load(polygons)
}
