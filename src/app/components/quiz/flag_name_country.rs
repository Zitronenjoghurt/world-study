use crate::app::components::quiz::QuizTrait;
use crate::app::persistence::persistent_object::PersistentObject;
use crate::get_data;
use eframe::emath::Vec2;
use egui::{Key, RichText, Ui};
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct FlagNameCountryQuiz {
    success: Option<bool>,
    started: bool,
    solution: Option<String>,
    answer: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FlagNameCountryQuizState {
    success: Option<bool>,
    started: bool,
    solution: Option<String>,
    answer: String,
}

impl PersistentObject for FlagNameCountryQuiz {
    type PersistentState = FlagNameCountryQuizState;

    fn save_state(&self) -> Self::PersistentState {
        FlagNameCountryQuizState {
            success: self.success,
            started: self.started,
            solution: self.solution.clone(),
            answer: self.answer.clone(),
        }
    }

    fn load_state(state: Self::PersistentState) -> Self {
        Self {
            success: state.success,
            started: state.started,
            solution: state.solution,
            answer: state.answer,
        }
    }
}

impl FlagNameCountryQuiz {
    pub fn submit(&mut self, country_code: &str) {
        let country = get_data().get_country(country_code).cloned().unwrap();
        self.success = Some(self.answer.to_lowercase() == country.name.to_lowercase());
    }
}

impl QuizTrait for FlagNameCountryQuiz {
    fn render(&mut self, ui: &mut Ui) -> Option<bool> {
        let country_code = self.solution.clone()?;

        ui.vertical_centered(|ui| {
            ui.add_space(40.0);

            ui.label(RichText::new("What country does this flag belong to?").size(30.0));
            ui.add_space(10.0);

            if let Some(flag_image) =
                get_data().get_country_flag_image(&country_code, Vec2::new(500.0, 500.0))
            {
                ui.add(flag_image);
            } else {
                ui.label("Flag not found");
            }

            ui.add_space(20.0);

            let text_edit_response = ui.text_edit_singleline(&mut self.answer);
            if text_edit_response.lost_focus() && ui.input(|input| input.key_pressed(Key::Enter)) {
                text_edit_response.request_focus();
                self.submit(&country_code);
            }

            ui.add_space(5.0);

            if ui.button("Submit").clicked() {
                self.submit(&country_code);
            }
        });

        self.success
    }

    fn render_settings(&mut self, ui: &mut Ui) {
        todo!()
    }

    fn start(&mut self) {
        self.started = true;

        let mut rng = rand::rng();
        let random_country_code = get_data()
            .get_country_codes()
            .choose(&mut rng)
            .unwrap()
            .clone();
        self.solution = Some(random_country_code);
    }

    fn has_started(&self) -> bool {
        self.started
    }

    fn is_successful(&self) -> Option<bool> {
        self.success
    }

    fn reset(&mut self) {
        self.success = None;
        self.started = false;
        self.solution = None;
        self.answer = String::new();
    }
}
