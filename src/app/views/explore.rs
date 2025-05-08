use crate::app::components::world_map::{WorldMapState, WorldMapStatePersist};
use crate::app::persistence::persistent_object::PersistentObject;
use crate::app::views::explore::big_flag::render_big_flag_window;
use crate::app::views::explore::debug::render_debug_window;
use crate::app::views::explore::details::render_details_window;
use crate::app::views::UIView;
use crate::app::WorldStudyApp;
use egui::Context;
use serde::{Deserialize, Serialize};

mod big_flag;
mod debug;
mod details;

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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExploreStatePersist {
    world_map: WorldMapStatePersist,
    debug_window_enabled: bool,
    details_window_enabled: bool,
    big_flag_window_enabled: bool,
}

impl PersistentObject for ExploreState {
    type PersistentState = ExploreStatePersist;

    fn save_state(&self) -> Self::PersistentState {
        ExploreStatePersist {
            world_map: self.world_map.save_state(),
            debug_window_enabled: self.debug_window_enabled,
            details_window_enabled: self.details_window_enabled,
            big_flag_window_enabled: self.big_flag_window_enabled,
        }
    }

    fn load_state(state: Self::PersistentState) -> Self {
        Self {
            world_map: WorldMapState::load_state(state.world_map),
            debug_window_enabled: state.debug_window_enabled,
            details_window_enabled: state.details_window_enabled,
            big_flag_window_enabled: state.big_flag_window_enabled,
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
