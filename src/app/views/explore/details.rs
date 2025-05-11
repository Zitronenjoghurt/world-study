use crate::app::WorldStudyApp;
use crate::get_data;
use egui::{Context, CursorIcon, Sense, Vec2};

const MAX_FLAG_WIDTH: f32 = 50.0;
const MAX_FLAG_HEIGHT: f32 = 50.0;

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
                        if let Some(flag_image) = get_data().get_country_flag_image(
                            selected_country,
                            Vec2::new(MAX_FLAG_WIDTH, MAX_FLAG_HEIGHT),
                        ) {
                            ui.strong("Flag");
                            let flag_response = ui.add(flag_image.sense(Sense::click()));
                            if flag_response.clicked() {
                                app.explore_state.big_flag_window_enabled = true;
                            }
                            if flag_response.hovered() {
                                flag_response.show_tooltip_text("Click to view full size");
                                ui.output_mut(|o| o.cursor_icon = CursorIcon::PointingHand);
                            }
                            ui.end_row();
                        }

                        ui.strong("Code");
                        ui.label(&country.code);
                        ui.end_row();

                        ui.strong("Official Name");
                        ui.label(&country.official_name);
                        ui.end_row();
                    })
            });
    }
}
