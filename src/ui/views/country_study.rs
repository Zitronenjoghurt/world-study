use crate::ui::components::world_map::WorldMapState;
use crate::ui::WorldStudyApp;
use egui::Context;

#[derive(Debug, Default)]
pub struct CountryStudyState {
    world_map: WorldMapState,
}

pub fn render(ctx: &Context, app: &mut WorldStudyApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        app.country_study_state.world_map.draw(ui);
    });
}
