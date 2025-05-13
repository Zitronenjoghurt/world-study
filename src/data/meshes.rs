use egui::{Color32, Pos2, Shape};
use geo::CoordsIter;
use std::collections::HashMap;
use std::sync::Arc;
use world_data::world_data_types::data::country::Country;

#[derive(Debug, Default)]
pub struct CountryMeshesMap(HashMap<String, CountryMeshes>);

impl CountryMeshesMap {
    pub fn get(&self, country_code: &str) -> Option<&CountryMeshes> {
        self.0.get(country_code)
    }

    pub fn build(countries: &HashMap<String, Arc<Country>>) -> Self {
        Self(CountryMeshes::build(countries))
    }
}

#[derive(Debug, Default)]
pub struct CountryMeshes {
    pub default: Vec<Shape>,
    pub hovered: Vec<Shape>,
    pub selected: Vec<Shape>,
}

impl CountryMeshes {
    pub fn build(countries: &HashMap<String, Arc<Country>>) -> HashMap<String, CountryMeshes> {
        let mut country_meshes = HashMap::new();

        for (code, country) in countries.iter() {
            let mut default_meshes = Vec::new();
            let mut hovered_meshes = Vec::new();
            let mut selected_meshes = Vec::new();

            for polygon in &country.polygons {
                let mut all_points = Vec::new();
                let mut flat_points = Vec::new();
                let mut hole_indices = Vec::new();

                let exterior = polygon.exterior();
                let exterior_point_count = exterior.coords_count();

                for coord in exterior.coords() {
                    let x = coord.x;
                    let y = -coord.y;
                    all_points.push(Pos2::new(x, y));
                    flat_points.push(x as f64);
                    flat_points.push(y as f64);
                }

                let mut current_index = exterior_point_count;
                for interior in polygon.interiors() {
                    hole_indices.push(current_index);

                    for coord in interior.coords() {
                        let x = coord.x;
                        let y = -coord.y;
                        all_points.push(Pos2::new(x, y));
                        flat_points.push(x as f64);
                        flat_points.push(y as f64);
                        current_index += 1;
                    }
                }

                if let Ok(indices) = earcutr::earcut(&flat_points, &hole_indices, 2) {
                    let create_mesh = |color: Color32| -> Shape {
                        let mut mesh = egui::Mesh::default();
                        mesh.vertices.reserve(all_points.len());

                        for point in &all_points {
                            mesh.vertices.push(egui::epaint::Vertex {
                                pos: *point,
                                uv: Pos2::ZERO,
                                color,
                            });
                        }

                        for chunk in indices.chunks(3) {
                            if chunk.len() == 3 {
                                mesh.indices.push(chunk[0] as u32);
                                mesh.indices.push(chunk[1] as u32);
                                mesh.indices.push(chunk[2] as u32);
                            }
                        }

                        Shape::Mesh(Arc::from(mesh))
                    };

                    default_meshes.push(create_mesh(Color32::from_rgb(222, 163, 139)));
                    hovered_meshes.push(create_mesh(Color32::from_rgb(249, 130, 132)));
                    selected_meshes.push(create_mesh(Color32::from_rgb(255, 196, 132)));
                }
            }

            country_meshes.insert(
                code.clone(),
                CountryMeshes {
                    default: default_meshes,
                    hovered: hovered_meshes,
                    selected: selected_meshes,
                },
            );
        }

        country_meshes
    }
}
