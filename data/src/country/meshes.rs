use crate::country::Country;
use crate::generic::data_map::DataMap;
use egui::{Color32, Pos2, Shape};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct CountryMeshesMap(HashMap<String, CountryMeshes>);

impl CountryMeshesMap {
    pub fn get(&self, country_code: &str) -> Option<&CountryMeshes> {
        self.0.get(country_code)
    }
}

impl CountryMeshesMap {
    pub fn build(countries: &DataMap<Country>) -> Self {
        Self(initialize_country_meshes(countries))
    }
}

#[derive(Debug, Default)]
pub struct CountryMeshes {
    pub default: Vec<Shape>,
    pub hovered: Vec<Shape>,
    pub selected: Vec<Shape>,
}

fn initialize_country_meshes(countries: &DataMap<Country>) -> HashMap<String, CountryMeshes> {
    let mut country_meshes = HashMap::new();

    for country in countries.iter() {
        let mut default_meshes = Vec::new();
        let mut hovered_meshes = Vec::new();
        let mut selected_meshes = Vec::new();

        for line in &country.outlines {
            let mut outline_points = Vec::new();
            let mut flat_points = Vec::new();
            for (x, y) in line {
                outline_points.push(Pos2::new(*x, *y));
                flat_points.push(*x as f64);
                flat_points.push(*y as f64);
            }

            if let Ok(indices) = earcutr::earcut(&flat_points, &[], 2) {
                let create_mesh = |color: Color32| -> Shape {
                    let mut mesh = egui::Mesh::default();
                    mesh.vertices.reserve(outline_points.len());

                    for point in &outline_points {
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
            country.code.clone(),
            CountryMeshes {
                default: default_meshes,
                hovered: hovered_meshes,
                selected: selected_meshes,
            },
        );
    }

    country_meshes
}
