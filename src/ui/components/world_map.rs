use crate::get_data;
use egui::{Color32, Pos2, Rect, Stroke, Ui, Vec2};
use std::sync::Arc;
use world_study_data::country::Country;

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
            for country in get_data().countries_iter() {
                draw_country(ui, country);
            }
        });
    }
}

fn draw_country(ui: &mut Ui, country: &Arc<Country>) {
    for line in &country.outlines {
        let mut points = Vec::new();
        for (x, y) in line {
            points.push(Pos2::new(*x, *y));
        }
        ui.painter()
            .line(points, Stroke::new(0.2, Color32::from_rgb(255, 255, 255)));
    }
}
