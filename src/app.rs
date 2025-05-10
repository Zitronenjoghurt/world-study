use crate::app::components::quiz::stats::QuizStats;
use crate::app::persistence::{persist_state, restore_state};
use crate::app::views::explore::{ExploreState, ExploreStatePersist};
use crate::app::views::main_menu::MainMenuState;
use crate::app::views::quiz_menu::{QuizMenuState, QuizMenuStatePersist};
use crate::app::views::quiz_run::{QuizRunState, QuizRunStatePersist};
use crate::app::views::study_menu::StudyMenuState;
use crate::app::views::*;
use eframe::{App, Frame};
use egui::Context;
use persistence::persistent_object::PersistentObject;
use serde::{Deserialize, Serialize};
use views::UIView;

mod components;
pub mod persistence;
mod styles;
mod views;

#[derive(Debug, Default)]
pub struct WorldStudyApp {
    current_view: UIView,
    quiz_history: Vec<QuizStats>,

    // View states
    main_menu_state: MainMenuState,
    study_menu_state: StudyMenuState,
    explore_state: ExploreState,
    quiz_menu_state: QuizMenuState,
    quiz_run_state: QuizRunState,
}

impl WorldStudyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(state) = restore_state() {
            Self::load_state(state)
        } else {
            Self::default()
        }
    }

    pub fn switch_view(&mut self, target_view: UIView) {
        self.current_view = target_view
    }

    pub fn log_quiz_stats(&mut self, stats: QuizStats) {
        self.quiz_history.push(stats);
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppState {
    last_view: UIView,
    quiz_history: Vec<QuizStats>,
    explore_state: ExploreStatePersist,
    quiz_menu_state: QuizMenuStatePersist,
    quiz_run_state: QuizRunStatePersist,
}

impl PersistentObject for WorldStudyApp {
    type PersistentState = AppState;

    fn save_state(&self) -> Self::PersistentState {
        AppState {
            last_view: self.current_view,
            quiz_history: self.quiz_history.clone(),
            explore_state: self.explore_state.save_state(),
            quiz_menu_state: self.quiz_menu_state.save_state(),
            quiz_run_state: self.quiz_run_state.save_state(),
        }
    }

    fn load_state(state: Self::PersistentState) -> Self {
        Self {
            current_view: state.last_view,
            quiz_history: state.quiz_history,
            main_menu_state: MainMenuState,
            study_menu_state: StudyMenuState,
            explore_state: ExploreState::load_state(state.explore_state),
            quiz_menu_state: QuizMenuState::load_state(state.quiz_menu_state),
            quiz_run_state: QuizRunState::load_state(state.quiz_run_state),
        }
    }
}

impl App for WorldStudyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        #[cfg(feature = "profiling")]
        profiling::scope!("frame");

        match self.current_view {
            UIView::MainMenu => main_menu::render(ctx, self),
            UIView::StudyMenu => study_menu::render(ctx, self),
            UIView::Explore => explore::render(ctx, self),
            UIView::QuizMenu => quiz_menu::render(ctx, self),
            UIView::QuizRun => quiz_run::render(ctx, self),
        }

        #[cfg(feature = "profiling")]
        profiling::finish_frame!();
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let state = self.save_state();
        persist_state(state);
    }
}
