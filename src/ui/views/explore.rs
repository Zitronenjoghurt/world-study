mod big_flag;
mod debug;
mod details;

use crate::ui::components::world_map::WorldMapState;
use crate::ui::views::explore::big_flag::render_big_flag_window;
use crate::ui::views::explore::debug::render_debug_window;
use crate::ui::views::explore::details::render_details_window;
use crate::ui::views::UIView;
use crate::ui::WorldStudyApp;
use egui::Context;

#[derive(Debug)]
pub struct ExploreState {
    world_map: WorldMapState,
    debug_window_enabled: bool,
    details_window_enabled: bool,
    big_flag_window_enabled: bool,
}

impl Default for ExploreState {
    fn default() -> Self {
        Self {
            world_map: WorldMapState::default(),
            debug_window_enabled: false,
            details_window_enabled: true,
            big_flag_window_enabled: false,
        }
    }
}

pub fn render(ctx: &Context, app: &mut WorldStudyApp) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            if ui.button(" 🏠 ").clicked() {
                app.switch_view(UIView::MainMenu);
            }

            ui.menu_button("Windows  ", |ui| {
                ui.checkbox(&mut app.explore_state.debug_window_enabled, "Debug");
                ui.checkbox(&mut app.explore_state.details_window_enabled, "Details");
            });
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        app.explore_state.world_map.draw(ui);
    });

    if app.explore_state.debug_window_enabled {
        render_debug_window(ctx, app);
    }

    if app.explore_state.details_window_enabled {
        render_details_window(ctx, app);
    }

    if app.explore_state.big_flag_window_enabled {
        render_big_flag_window(ctx, app);
    }
}
