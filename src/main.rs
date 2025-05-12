use crate::app::WorldStudyApp;
use crate::data::WorldStudyData;
use once_cell::sync::Lazy;
use std::sync::Arc;

mod app;
mod data;
pub mod utils;

static DATA: Lazy<Arc<WorldStudyData>> = Lazy::new(|| Arc::new(WorldStudyData::load()));

pub fn get_data() -> Arc<WorldStudyData> {
    DATA.clone()
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "World Study",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(WorldStudyApp::new(cc)))
        }),
    )
    .expect("Failed to run egui application.");
}
