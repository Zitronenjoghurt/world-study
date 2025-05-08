use crate::app::components::world_map::WorldMapState;
use crate::app::WorldStudyApp;
use egui::Context;

#[derive(Debug, Default)]
pub struct StudyMenuState {
    world_map: WorldMapState,
}

pub fn render(ctx: &Context, app: &mut WorldStudyApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        app.study_menu_state.world_map.draw(ui);
    });
}
