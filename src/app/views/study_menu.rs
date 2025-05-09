use crate::app::views::UIView;
use crate::app::WorldStudyApp;
use egui::Context;

#[derive(Debug, Default)]
pub struct StudyMenuState;

pub fn render(ctx: &Context, app: &mut WorldStudyApp) {
    egui::TopBottomPanel::top("quiz_menu_top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            if ui.button(" 🏠 ").clicked() {
                app.switch_view(UIView::MainMenu);
            }
        });
    });
}
