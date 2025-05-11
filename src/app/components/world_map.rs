use crate::app::persistence::persistent_object::PersistentObject;
use crate::get_data;
use egui::{Color32, Pos2, Rect, Stroke, Ui, Vec2};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WorldMapStatePersist {
    scene_rect_min_x: f32,
    scene_rect_min_y: f32,
    scene_rect_max_x: f32,
    scene_rect_max_y: f32,
    selected_country: Option<String>,
}

impl PersistentObject for WorldMapState {
    type PersistentState = WorldMapStatePersist;

    fn save_state(&self) -> Self::PersistentState {
        WorldMapStatePersist {
            scene_rect_min_x: self.scene_rect.min.x,
            scene_rect_min_y: self.scene_rect.min.y,
            scene_rect_max_x: self.scene_rect.max.x,
            scene_rect_max_y: self.scene_rect.max.y,
            selected_country: self.selected_country.clone(),
        }
    }

    fn load_state(state: Self::PersistentState) -> Self {
        let min = Pos2::new(state.scene_rect_min_x, state.scene_rect_min_y);
        let max = Pos2::new(state.scene_rect_max_x, state.scene_rect_max_y);
        let scene_rect = Rect::from_min_max(min, max);
        Self {
            scene_rect,
            hovered_country: None,
            selected_country: state.selected_country,
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

    if let Some(capitals) = get_data().get_country_capitals(country_code) {
        for capital in capitals {
            let (_, pos) = capital;
            let position = Pos2::new(pos.x, pos.y);
            ui.painter().circle(
                position,
                0.25,
                Color32::from_rgb(255, 0, 0),
                Stroke::new(0.025, Color32::from_rgb(0, 0, 0)),
            );
        }
    }
}
