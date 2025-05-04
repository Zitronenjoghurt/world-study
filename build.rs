use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use world_study_data::WorldStudyData;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("data.bin");

    let data = WorldStudyData::build();
    let encoded_data = bincode::serialize(&data).unwrap();

    let mut compressor = ZlibEncoder::new(Vec::new(), Compression::best());
    compressor.write_all(&encoded_data).unwrap();
    let compressed_data = compressor.finish().unwrap();

    let mut file = File::create(dest_path).unwrap();
    file.write_all(&compressed_data).unwrap();
}
