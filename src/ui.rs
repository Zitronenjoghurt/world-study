use crate::ui::views::country_study::CountryStudyState;
use crate::ui::views::explore::ExploreState;
use crate::ui::views::main_menu::MainMenuState;
use crate::ui::views::*;
use eframe::{App, Frame};
use egui::Context;
use views::UIView;

mod components;
mod styles;
mod views;

#[derive(Debug, Default)]
pub struct WorldStudyApp {
    current_view: UIView,

    // View states
    main_menu_state: MainMenuState,
    country_study_state: CountryStudyState,
    explore_state: ExploreState,
}

impl WorldStudyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    pub fn switch_view(&mut self, target_view: UIView) {
        self.current_view = target_view
    }
}

impl App for WorldStudyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        #[cfg(feature = "profiling")]
        profiling::scope!("frame");

        match self.current_view {
            UIView::MainMenu => main_menu::render(ctx, self),
            UIView::CountryStudy => country_study::render(ctx, self),
            UIView::Explore => explore::render(ctx, self),
        }

        #[cfg(feature = "profiling")]
        profiling::finish_frame!();
    }
}
