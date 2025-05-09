use crate::app::components::quiz::settings::QuizSettingsTrait;
use crate::app::components::quiz::types::flag_name_country::FlagNameCountryQuiz;
use crate::app::components::quiz::Quiz;
use crate::app::components::settings_slider::SettingsSlider;
use egui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagNameCountryQuizSettings {
    pub max_tries: u8,
}

impl Default for FlagNameCountryQuizSettings {
    fn default() -> Self {
        Self { max_tries: 1 }
    }
}

impl QuizSettingsTrait for FlagNameCountryQuizSettings {
    fn render(&mut self, ui: &mut Ui) {
        SettingsSlider::build()
            .text("Tries")
            .tooltip("How often you can try guessing the flag's country.")
            .draw(ui, &mut self.max_tries, 1, 1..=10, 1.0);
    }

    fn create_quiz(&self) -> Quiz {
        Quiz::FlagNameCountry(FlagNameCountryQuiz::with_settings(self.clone()))
    }
}
