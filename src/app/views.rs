use serde::{Deserialize, Serialize};

pub mod explore;
pub mod main_menu;
pub mod quiz_menu;
pub mod quiz_run;
pub mod study_menu;

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub enum UIView {
    #[default]
    MainMenu,
    StudyMenu,
    Explore,
    QuizMenu,
    QuizRun,
}
