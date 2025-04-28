use crate::ui::WorldStudyApp;

mod ui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "World Study",
        native_options,
        Box::new(|cc| Ok(Box::new(WorldStudyApp::new(cc)))),
    )
    .expect("Failed to run egui application.");
}
