use crate::get_data;
use egui::{Color32, Pos2, Rect, Ui, Vec2};

const HEIGHT: f32 = 670.0;
const WIDTH: f32 = 1010.0;
const BACKGROUND_COLOR: Color32 = Color32::from_rgb(172, 204, 228);
const CORNER_RADIUS: f32 = 10.0;

#[derive(Debug)]
pub struct WorldMapState {
    scene_rect: Rect,
    pub hovered_country: Option<String>,
    pub selected_country: Option<String>,
    pub mouse_position: Option<Pos2>,
}

impl Default for WorldMapState {
    fn default() -> Self {
        Self {
            scene_rect: Rect::from_min_size(Pos2::ZERO, Vec2::new(WIDTH, HEIGHT)),
            hovered_country: None,
            selected_country: None,
            mouse_position: None,
        }
    }
}

impl WorldMapState {
    pub fn draw(&mut self, ui: &mut Ui) {
        let scene = egui::Scene::new().zoom_range(0.2..=1000.0);

        scene.show(ui, &mut self.scene_rect, |ui| {
            let hover_rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(WIDTH, HEIGHT));
            ui.painter()
                .rect_filled(hover_rect, CORNER_RADIUS, BACKGROUND_COLOR);

            let hover_rect_response =
                ui.interact(hover_rect, ui.id().with("map_area"), egui::Sense::click());
            if hover_rect_response.hovered() {
                if let Some(mouse_pos) = hover_rect_response.hover_pos() {
                    self.hovered_country =
                        get_data().get_country_code_at_point(mouse_pos.x, mouse_pos.y);
                    self.mouse_position = Some(mouse_pos);
                }
            }

            if hover_rect_response.clicked() {
                if let Some(hovered_country) = &self.hovered_country {
                    self.selected_country =
                        if self.selected_country.as_ref() == Some(hovered_country) {
                            None
                        } else {
                            Some(hovered_country.to_owned())
                        };
                } else {
                    self.selected_country = None;
                }
            }

            for country_code in get_data().get_country_codes() {
                let is_selected = Some(country_code.to_owned()) == self.selected_country;
                let is_hovered = Some(country_code.to_owned()) == self.hovered_country;
                draw_country(ui, country_code, is_selected, is_hovered);
            }
        });
    }
}

fn draw_country(ui: &mut Ui, country_code: &str, is_selected: bool, is_hovered: bool) {
    #[cfg(feature = "profiling")]
    profiling::scope!("draw_country");

    if let Some(country_meshes) = get_data().get_country_meshes(country_code) {
        let meshes = if is_selected {
            &country_meshes.selected
        } else if is_hovered {
            &country_meshes.hovered
        } else {
            &country_meshes.default
        };

        for mesh in meshes {
            ui.painter().add(mesh.clone());
        }
    }

    if let Some(outlines) = get_data().get_country_outlines(country_code) {
        for outline in outlines {
            ui.painter().add(outline.clone());
        }
    }
}
