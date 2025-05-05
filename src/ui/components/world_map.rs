use crate::get_data;
use egui::{Pos2, Rect, Ui, Vec2};

#[derive(Debug)]
pub struct WorldMapState {
    scene_rect: Rect,
}

impl Default for WorldMapState {
    fn default() -> Self {
        Self::new(500.0, 500.0)
    }
}

impl WorldMapState {
    pub fn new(size_x: f32, size_y: f32) -> Self {
        Self {
            scene_rect: Rect::from_min_size(Pos2::ZERO, Vec2::new(size_x, size_y)),
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        let scene = egui::Scene::new()
            .zoom_range(0.1..=10.0)
            .max_inner_size(Vec2::new(200.0, 200.0));

        scene.show(ui, &mut self.scene_rect, |ui| {
            for country_code in get_data().get_country_codes() {
                draw_country(ui, country_code);
            }
        });
    }
}

fn draw_country(ui: &mut Ui, country_code: &str) {
    if let Some(outlines) = get_data().get_country_outlines(country_code) {
        for outline in outlines {
            ui.painter().add(outline.clone());
        }
    }

    if let Some(meshes) = get_data().get_country_meshes(country_code) {
        for mesh in meshes {
            ui.painter().add(mesh.clone());
        }
    }
}
