use crate::ui::WorldStudyApp;
use flate2::read::ZlibDecoder;
use once_cell::sync::Lazy;
use std::io::Read;
use std::sync::Arc;
use world_study_data::WorldStudyData;

mod ui;

const INCLUDED_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/data.bin"));
static DATA: Lazy<Arc<WorldStudyData>> = Lazy::new(|| {
    let mut decompressor = ZlibDecoder::new(INCLUDED_DATA);
    let mut decompressed_data = Vec::new();
    decompressor.read_to_end(&mut decompressed_data).unwrap();

    let mut data: WorldStudyData = bincode::deserialize(&decompressed_data).unwrap();
    data.initialize();
    Arc::new(data)
});

pub fn get_data() -> Arc<WorldStudyData> {
    DATA.clone()
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "World Study",
        native_options,
        Box::new(|cc| Ok(Box::new(WorldStudyApp::new(cc)))),
    )
    .expect("Failed to run egui application.");
}
