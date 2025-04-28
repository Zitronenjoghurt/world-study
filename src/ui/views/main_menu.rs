use crate::ui::components::custom_button::CustomButton;
use crate::ui::styles::generic_size::GenericSize;
use crate::ui::WorldStudyApp;
use egui::{Context, Response, RichText, Ui, Vec2};

#[derive(Debug, Default)]
pub struct MainMenuState;

pub fn render(ctx: &Context, _app: &mut WorldStudyApp) {
    egui::Window::new(RichText::new("World Study").size(30.0))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .fixed_size(Vec2::new(200.0, 100.0))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                menu_buttons(ui);
            });
        });
}

fn menu_buttons(ui: &mut Ui) {
    menu_button(ui, "Study");
}

fn menu_button(ui: &mut Ui, text: &str) -> Response {
    CustomButton::new(text)
        .size(GenericSize::XL)
        .min_width(150.0)
        .draw(ui)
}
