use crate::app::components::quiz::settings::flag_name_country::FlagNameCountryQuizSettings;
use crate::app::components::quiz::settings::QuizSettingsTrait;
use crate::app::components::quiz::Quiz;
use crate::app::persistence::persistent_object::PersistentObject;
use crate::app::views::UIView;
use crate::app::WorldStudyApp;
use egui::{Context, RichText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct QuizMenuState {
    flag_name_quiz_settings: FlagNameCountryQuizSettings,
    flag_name_quiz_enabled: bool,
}

impl QuizMenuState {
    pub fn build_quizzes(&self) -> Vec<Quiz> {
        let mut quizzes = Vec::new();
        if self.flag_name_quiz_enabled {
            quizzes.push(self.flag_name_quiz_settings.create_quiz());
        }
        quizzes
    }

    pub fn has_at_least_one_quiz_enabled(&self) -> bool {
        self.flag_name_quiz_enabled
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QuizMenuStatePersist {
    flag_name_quiz_settings: FlagNameCountryQuizSettings,
    flag_name_quiz_enabled: bool,
}

impl PersistentObject for QuizMenuState {
    type PersistentState = QuizMenuStatePersist;

    fn save_state(&self) -> Self::PersistentState {
        QuizMenuStatePersist {
            flag_name_quiz_settings: self.flag_name_quiz_settings.clone(),
            flag_name_quiz_enabled: self.flag_name_quiz_enabled,
        }
    }

    fn load_state(state: Self::PersistentState) -> Self {
        Self {
            flag_name_quiz_settings: state.flag_name_quiz_settings,
            flag_name_quiz_enabled: state.flag_name_quiz_enabled,
        }
    }
}

pub fn render(ctx: &Context, app: &mut WorldStudyApp) {
    egui::TopBottomPanel::top("quiz_menu_top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            if ui.button(" 🏠 ").clicked() {
                app.switch_view(UIView::MainMenu);
            }
            ui.menu_button("Quizzes", |ui| {
                ui.checkbox(
                    &mut app.quiz_menu_state.flag_name_quiz_enabled,
                    "Flag ➡ Country",
                );
            });
        });
    });

    egui::Window::new("Quiz Settings")
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let start_button = ui.add_enabled(
                    app.quiz_menu_state.has_at_least_one_quiz_enabled(),
                    egui::Button::new(RichText::new("Start").size(20.0)),
                );

                if start_button.clicked() {
                    let quizzes = app.quiz_menu_state.build_quizzes();
                    app.quiz_run_state.load_quizzes(quizzes);
                    app.switch_view(UIView::QuizRun);
                }
            });
        });

    if app.quiz_menu_state.flag_name_quiz_enabled {
        egui::Window::new("Flag ➡ Country")
            .open(&mut app.quiz_menu_state.flag_name_quiz_enabled)
            .show(ctx, |ui| {
                app.quiz_menu_state.flag_name_quiz_settings.render(ui);
            });
    }

    //egui::CentralPanel::default().show(ctx, |ui| match app.quiz_menu_state.quiz.render(ui) {
    //    None => {}
    //    _ => app.quiz_menu_state.quiz.start(),
    //});
}
