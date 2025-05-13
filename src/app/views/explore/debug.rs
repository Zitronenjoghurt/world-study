use crate::app::WorldStudyApp;
use egui::{Context, Id};

pub fn render_debug_window(ctx: &Context, app: &mut WorldStudyApp) {
    egui::Window::new("Debug")
        .id(Id::new("explore_debug_window"))
        .open(&mut app.explore_state.debug_window_enabled)
        .show(ctx, |ui| {
            if let Some(mouse_pos) = app.explore_state.world_map.mouse_position {
                ui.label(format!("pointer: {:.2}, {:.2}", mouse_pos.x, mouse_pos.y));
            }

            if let Some(hovered) = &app.explore_state.world_map.hovered_country {
                ui.label(format!("hovered: {hovered}"));
            } else {
                ui.label("hovered country: None");
            }

            if let Some(selected) = &app.explore_state.world_map.selected_country {
                ui.label(format!("selected: {selected}"));
            } else {
                ui.label("selected country: None");
            }

            if let Some(hovered) = &app.explore_state.world_map.hovered_capital {
                ui.label(format!("hovered: {hovered}"));
            } else {
                ui.label("hovered capital: None");
            }

            if let Some(selected) = &app.explore_state.world_map.selected_capital {
                ui.label(format!("selected: {selected}"));
            } else {
                ui.label("selected capital: None");
            }

            ui.label(format!(
                "update: {}ms",
                app.update_time.as_micros() as f64 / 1000.0
            ))
        });
}
