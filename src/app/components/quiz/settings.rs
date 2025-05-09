use crate::app::components::quiz::{Quiz, QuizTrait};
use egui::Ui;

pub mod flag_name_country;

pub trait QuizSettingsTrait: Sized {
    fn render(&mut self, ui: &mut Ui);
    fn create_quiz(&self) -> Quiz;
}
