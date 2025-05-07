use crate::get_data;
use crate::ui::WorldStudyApp;
use egui::{Context, Vec2};

const MAX_FLAG_WIDTH: f32 = 100.0;
const MAX_FLAG_HEIGHT: f32 = 100.0;

pub fn render_details_window(ctx: &Context, app: &mut WorldStudyApp) {
    if let Some(selected_country) = &app.explore_state.world_map.selected_country {
        let country = &get_data().get_country(selected_country).unwrap().clone();

        egui::Window::new(&country.name)
            .open(&mut app.explore_state.details_window_enabled)
            .id(egui::Id::new("explore_details_window"))
            .show(ctx, |ui| {
                egui::Grid::new("country_details")
                    .striped(true)
                    .show(ui, |ui| {
                        ui.strong("Flag");
                        ui.add(
                            get_data()
                                .get_country_flag_image(
                                    selected_country,
                                    Vec2::new(MAX_FLAG_WIDTH, MAX_FLAG_HEIGHT),
                                )
                                .unwrap(),
                        );
                        ui.end_row();

                        ui.strong("Code");
                        ui.label(&country.code);
                        ui.end_row();

                        ui.strong("Long Name");
                        ui.label(&country.long_name);
                        ui.end_row();
                    })
            });
    }
}
