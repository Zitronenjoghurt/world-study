use crate::app::components::quiz::{Quiz, QuizState, QuizTrait};
use crate::app::persistence::persistent_object::PersistentObject;
use crate::app::views::UIView;
use crate::app::WorldStudyApp;
use egui::{Context, Key};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct QuizRunState {
    quizzes: Vec<Quiz>,
    stats_collected: bool,
    active_quiz: usize,
}

impl QuizRunState {
    pub fn load_quizzes(&mut self, quizzes: Vec<Quiz>) {
        self.quizzes = quizzes;
        self.randomize_quiz();
    }

    pub fn randomize_quiz(&mut self) {
        let mut rng = rand::rng();
        self.active_quiz = rng.random_range(0..self.quizzes.len());
        self.restart_active_quiz();
    }

    pub fn get_active_quiz(&mut self) -> &mut Quiz {
        &mut self.quizzes[self.active_quiz]
    }

    pub fn restart_active_quiz(&mut self) {
        self.stats_collected = false;
        self.get_active_quiz().start()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QuizRunStatePersist {
    quizzes: Vec<QuizState>,
    stats_collected: bool,
    active_quiz: usize,
}

impl PersistentObject for QuizRunState {
    type PersistentState = QuizRunStatePersist;

    fn save_state(&self) -> Self::PersistentState {
        QuizRunStatePersist {
            quizzes: self.quizzes.iter().map(|q| q.save_state()).collect(),
            stats_collected: self.stats_collected,
            active_quiz: self.active_quiz,
        }
    }

    fn load_state(state: Self::PersistentState) -> Self {
        Self {
            quizzes: state.quizzes.into_iter().map(Quiz::load_state).collect(),
            stats_collected: state.stats_collected,
            active_quiz: state.active_quiz,
        }
    }
}

pub fn render(ctx: &Context, app: &mut WorldStudyApp) {
    egui::TopBottomPanel::top("quiz_run_top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            if ui.button(" ⬅ ").clicked() {
                app.switch_view(UIView::QuizMenu);
            }
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        if app.quiz_run_state.get_active_quiz().render(ui).is_some() {
            if !app.quiz_run_state.stats_collected {
                if let Some(stats) = app.quiz_run_state.get_active_quiz().collect_stats() {
                    app.log_quiz_stats(stats);
                    app.quiz_run_state.stats_collected = true;
                }
            }

            ui.add_space(5.0);
            let next_button = ui.vertical_centered(|ui| ui.button("Next"));
            if next_button.inner.clicked() || ui.input(|input| input.key_pressed(Key::Space)) {
                app.quiz_run_state.randomize_quiz();
            }
        }
    });
}
