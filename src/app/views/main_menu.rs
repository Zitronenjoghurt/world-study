use crate::app::components::custom_button::CustomButton;
use crate::app::styles::generic_size::GenericSize;
use crate::app::views::UIView;
use crate::app::WorldStudyApp;
use egui::{Context, Response, RichText, Ui, Vec2};

#[derive(Debug, Default)]
pub struct MainMenuState;

pub fn render(ctx: &Context, app: &mut WorldStudyApp) {
    egui::Window::new(RichText::new("World Study").size(30.0))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .fixed_size(Vec2::new(200.0, 100.0))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                menu_buttons(ui, app);
            });
        });
}

fn menu_buttons(ui: &mut Ui, app: &mut WorldStudyApp) {
    if menu_button(ui, "Quiz").clicked() {
        app.switch_view(UIView::QuizMenu);
    }
    if menu_button(ui, "Study").clicked() {
        app.switch_view(UIView::StudyMenu);
    }
    if menu_button(ui, "Explore").clicked() {
        app.switch_view(UIView::Explore);
    }
}

fn menu_button(ui: &mut Ui, text: &str) -> Response {
    CustomButton::new(text)
        .size(GenericSize::XL)
        .min_width(150.0)
        .draw(ui)
}
