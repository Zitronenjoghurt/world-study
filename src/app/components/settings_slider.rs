use crate::app::components::value_reset_button::draw_value_reset_button;
use egui::{emath, Ui, WidgetText};
use std::ops::RangeInclusive;

#[derive(Default)]
pub struct SettingsSlider {
    text: Option<WidgetText>,
    tooltip: Option<WidgetText>,
    logarithmic: bool,
    prefix: Option<String>,
    suffix: Option<String>,
}

impl SettingsSlider {
    pub fn build() -> Self {
        Self::default()
    }

    pub fn draw<Num: emath::Numeric>(
        &self,
        ui: &mut Ui,
        value: &mut Num,
        default_value: Num,
        range: RangeInclusive<Num>,
        step: f64,
    ) {
        ui.horizontal(|ui| {
            draw_value_reset_button(ui, value, default_value);

            let mut slider = egui::Slider::new(value, range).step_by(step);
            if self.logarithmic {
                slider = slider.logarithmic(true);
            }
            if let Some(prefix) = &self.prefix {
                slider = slider.prefix(prefix);
            }
            if let Some(suffix) = &self.suffix {
                slider = slider.suffix(suffix);
            }

            ui.add(slider);

            if let Some(text) = self.text.clone() {
                let label = ui.label(text);
                if let Some(tooltip) = self.tooltip.clone() {
                    label.on_hover_text(tooltip);
                }
            }
        });
    }

    pub fn text(&mut self, text: impl Into<WidgetText>) -> &mut Self {
        self.text = Some(text.into());
        self
    }

    pub fn tooltip(&mut self, tooltip: impl Into<WidgetText>) -> &mut Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn logarithmic(&mut self, logarithmic: bool) -> &mut Self {
        self.logarithmic = logarithmic;
        self
    }

    pub fn prefix(&mut self, prefix: impl Into<String>) -> &mut Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn suffix(&mut self, suffix: impl Into<String>) -> &mut Self {
        self.suffix = Some(suffix.into());
        self
    }
}
