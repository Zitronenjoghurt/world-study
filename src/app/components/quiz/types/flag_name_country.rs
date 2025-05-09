use crate::app::components::quiz::settings::flag_name_country::FlagNameCountryQuizSettings;
use crate::app::components::quiz::QuizTrait;
use crate::app::persistence::persistent_object::PersistentObject;
use crate::get_data;
use eframe::emath::Vec2;
use egui::{Align, Key, Layout, RichText, Ui};
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct FlagNameCountryQuiz {
    success: Option<bool>,
    started: bool,
    solution: Option<String>,
    answer: String,
    tries: u8,
    settings: FlagNameCountryQuizSettings,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FlagNameCountryQuizState {
    success: Option<bool>,
    started: bool,
    solution: Option<String>,
    answer: String,
    tries: u8,
    settings: FlagNameCountryQuizSettings,
}

impl PersistentObject for FlagNameCountryQuiz {
    type PersistentState = FlagNameCountryQuizState;

    fn save_state(&self) -> Self::PersistentState {
        FlagNameCountryQuizState {
            success: self.success,
            started: self.started,
            solution: self.solution.clone(),
            answer: self.answer.clone(),
            tries: self.tries,
            settings: self.settings.clone(),
        }
    }

    fn load_state(state: Self::PersistentState) -> Self {
        Self {
            success: state.success,
            started: state.started,
            solution: state.solution,
            answer: state.answer,
            tries: state.tries,
            settings: state.settings,
        }
    }
}

impl FlagNameCountryQuiz {
    pub fn with_settings(settings: FlagNameCountryQuizSettings) -> Self {
        Self {
            settings,
            ..Self::default()
        }
    }

    fn tries_left(&self) -> u8 {
        self.settings.max_tries - self.tries
    }

    fn submit(&mut self, country_code: &str) {
        let country = get_data().get_country(country_code).cloned().unwrap();
        let right_answer = self.answer.to_lowercase() == country.name.to_lowercase();

        self.tries += 1;
        if right_answer {
            self.success = Some(true);
        } else if self.tries >= self.settings.max_tries {
            self.success = Some(false);
        } else {
            self.answer.clear();
        }
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

            match self.success {
                None => {
                    ui.add_space(20.0);

                    let text_edit_response = ui.text_edit_singleline(&mut self.answer);
                    if text_edit_response.lost_focus()
                        && ui.input_mut(|input| input.key_pressed(Key::Enter))
                    {
                        self.submit(&country_code);
                    }
                    text_edit_response.request_focus();

                    ui.add_space(5.0);

                    ui.columns(2, |columns| {
                        columns[0].allocate_ui_with_layout(
                            Vec2::ZERO,
                            Layout::right_to_left(Align::Center),
                            |ui| {
                                if ui.button("Submit").clicked() {
                                    self.submit(&country_code);
                                }
                                if self.settings.max_tries > 1 {
                                    ui.label(format!("Tries left: {}", self.tries_left()));
                                }
                            },
                        );
                        columns[1].allocate_ui_with_layout(
                            Vec2::ZERO,
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                if ui.button("Dunno").clicked() {
                                    self.success = Some(false);
                                }
                            },
                        );
                    });
                }
                Some(true) => {
                    ui.add_space(20.0);
                    ui.label("Correct!");
                }
                Some(false) => {
                    let country = get_data().get_country(&country_code).unwrap().clone();
                    ui.add_space(20.0);
                    ui.label(format!("Solution: {}", country.name));
                }
            }
        });

        self.success
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
        self.tries = 0;
    }
}
