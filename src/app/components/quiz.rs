use crate::app::persistence::persistent_object::PersistentObject;
use egui::Ui;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use types::flag_name_country::{FlagNameCountryQuiz, FlagNameCountryQuizState};

pub mod settings;
pub mod types;

pub trait QuizTrait: Debug + Default + PersistentObject {
    fn render(&mut self, ui: &mut Ui) -> Option<bool>;
    fn start(&mut self);
    fn has_started(&self) -> bool;
    fn is_successful(&self) -> Option<bool>;
    fn reset(&mut self);
}

#[derive(Debug)]
pub enum Quiz {
    FlagNameCountry(FlagNameCountryQuiz),
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum QuizType {
    #[default]
    FlagNameCountry,
}

impl Default for Quiz {
    fn default() -> Self {
        Self::FlagNameCountry(FlagNameCountryQuiz::default())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QuizState {
    FlagNameCountry(FlagNameCountryQuizState),
}

impl Default for QuizState {
    fn default() -> Self {
        Self::FlagNameCountry(FlagNameCountryQuizState::default())
    }
}

impl PersistentObject for Quiz {
    type PersistentState = QuizState;

    fn save_state(&self) -> Self::PersistentState {
        match self {
            Self::FlagNameCountry(quiz) => QuizState::FlagNameCountry(quiz.save_state()),
        }
    }

    fn load_state(state: Self::PersistentState) -> Self {
        match state {
            QuizState::FlagNameCountry(state) => {
                Self::FlagNameCountry(FlagNameCountryQuiz::load_state(state))
            }
        }
    }
}

impl QuizTrait for Quiz {
    fn render(&mut self, ui: &mut Ui) -> Option<bool> {
        if !self.has_started() {
            self.start();
        }

        match self {
            Self::FlagNameCountry(quiz) => quiz.render(ui),
        }
    }

    fn start(&mut self) {
        self.reset();

        match self {
            Self::FlagNameCountry(quiz) => quiz.start(),
        }
    }

    fn has_started(&self) -> bool {
        match self {
            Self::FlagNameCountry(quiz) => quiz.has_started(),
        }
    }

    fn is_successful(&self) -> Option<bool> {
        match self {
            Self::FlagNameCountry(quiz) => quiz.is_successful(),
        }
    }

    fn reset(&mut self) {
        match self {
            Self::FlagNameCountry(quiz) => quiz.reset(),
        }
    }
}
