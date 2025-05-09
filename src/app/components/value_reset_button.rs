use egui::{Response, RichText, Ui};

pub fn draw_value_reset_button<V: PartialEq>(
    ui: &mut Ui,
    value: &mut V,
    default_value: V,
) -> Response {
    let is_default = *value == default_value;

    let reset_button = ui.add_enabled(
        !is_default,
        egui::Button::new(RichText::new("↺").size(12.0))
            .frame(false)
            .small(),
    );

    if reset_button.clicked() {
        *value = default_value;
    }

    if reset_button.hovered() {
        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
    }

    reset_button
}
