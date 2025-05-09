use crate::app::components::quiz::{Quiz, QuizState, QuizTrait};
use crate::app::persistence::persistent_object::PersistentObject;
use crate::app::views::UIView;
use crate::app::WorldStudyApp;
use egui::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct QuizMenuState {
    quiz: Quiz,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QuizMenuStatePersist {
    quiz: QuizState,
}

impl PersistentObject for QuizMenuState {
    type PersistentState = QuizMenuStatePersist;

    fn save_state(&self) -> Self::PersistentState {
        QuizMenuStatePersist {
            quiz: self.quiz.save_state(),
        }
    }

    fn load_state(state: Self::PersistentState) -> Self {
        Self {
            quiz: Quiz::load_state(state.quiz),
        }
    }
}

pub fn render(ctx: &Context, app: &mut WorldStudyApp) {
    egui::TopBottomPanel::top("quiz_menu_top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            if ui.button(" 🏠 ").clicked() {
                app.switch_view(UIView::MainMenu);
            }
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| match app.quiz_menu_state.quiz.render(ui) {
        None => {}
        _ => app.quiz_menu_state.quiz.start(),
    });
}
