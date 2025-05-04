pub mod country_study;
pub mod main_menu;

#[derive(Debug, Default, Copy, Clone)]
pub enum UIView {
    #[default]
    MainMenu,
    CountryStudy,
}
