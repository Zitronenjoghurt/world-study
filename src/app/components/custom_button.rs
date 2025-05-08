use crate::app::styles::button_size::ButtonSize;
use crate::app::styles::generic_size::GenericSize;
use crate::app::styles::text_size::TextSize;
use egui::{Button, Response, RichText, Ui};

#[derive(Debug)]
pub struct CustomButton {
    text: String,
    size: GenericSize,
    min_width: f32,
}

impl Default for CustomButton {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            size: GenericSize::M,
            min_width: 0.0,
        }
    }
}

impl CustomButton {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn size(mut self, size: GenericSize) -> Self {
        self.size = size;
        self
    }

    pub fn min_width(mut self, min_width: f32) -> Self {
        self.min_width = min_width;
        self
    }

    pub fn draw(self, ui: &mut Ui) -> Response {
        let text_size = TextSize::from(self.size).size();
        let button_size = ButtonSize::from(self.size).size(self.min_width);

        let button = Button::new(RichText::new(&self.text).size(text_size)).min_size(button_size);

        ui.add(button)
    }
}
