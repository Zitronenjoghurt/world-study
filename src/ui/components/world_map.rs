use crate::get_data;
use egui::{Pos2, Rect, Ui, Vec2};

#[derive(Debug)]
pub struct WorldMapState {
    scene_rect: Rect,
    hovered_country: Option<String>,
}

impl Default for WorldMapState {
    fn default() -> Self {
        Self {
            scene_rect: Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 1000.0)),
            hovered_country: None,
        }
    }
}

impl WorldMapState {
    pub fn draw(&mut self, ui: &mut Ui) {
        let scene = egui::Scene::new()
            .zoom_range(0.1..=20.0)
            .max_inner_size(Vec2::new(200.0, 200.0));

        scene.show(ui, &mut self.scene_rect, |ui| {
            let hover_rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(1000.0, 1000.0));
            let hover_rect_response = ui.allocate_rect(hover_rect, egui::Sense::hover());

            if hover_rect_response.hovered() {
                if let Some(mouse_pos) = hover_rect_response.hover_pos() {
                    self.hovered_country =
                        get_data().get_country_code_at_point(mouse_pos.x, mouse_pos.y);
                    println!("{:?}", self.hovered_country);
                }
            }

            for country_code in get_data().get_country_codes() {
                let is_hovered = Some(country_code.to_owned()) == self.hovered_country;
                draw_country(ui, country_code, is_hovered);
            }
        });
    }
}

fn draw_country(ui: &mut Ui, country_code: &str, is_hovered: bool) {
    if let Some(outlines) = get_data().get_country_outlines(country_code) {
        for outline in outlines {
            ui.painter().add(outline.clone());
        }
    }

    if let Some(meshes) = get_data().get_country_meshes(country_code) {
        if is_hovered {
            for mesh in &meshes.hovered {
                ui.painter().add(mesh.clone());
            }
        } else {
            for mesh in &meshes.default {
                ui.painter().add(mesh.clone());
            }
        }
    }
}
