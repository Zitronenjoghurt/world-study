use crate::app::WorldStudyApp;
use crate::get_data;
use eframe::emath::Vec2;
use egui::{Context, Id};

const DEFAULT_HEIGHT: f32 = 500.0;
const DEFAULT_WIDTH: f32 = 500.0;

pub fn render_big_flag_window(ctx: &Context, app: &mut WorldStudyApp) {
    let Some(selected_country) = &app.explore_state.world_map.selected_country else {
        return;
    };

    let Some(country) = &get_data().get_country(selected_country).cloned() else {
        return;
    };

    egui::Window::new(format!("Flag of {}", country.name))
        .id(Id::new("explore_big_flag_window"))
        .open(&mut app.explore_state.big_flag_window_enabled)
        .resizable(true)
        .show(ctx, |ui| {
            if let Some(flag_image) = get_data()
                .get_country_flag_image(selected_country, Vec2::new(DEFAULT_HEIGHT, DEFAULT_WIDTH))
            {
                ui.add(flag_image);
            } else {
                ui.label("Flag not found");
            }
        });
}
