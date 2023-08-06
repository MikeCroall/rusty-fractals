use crate::size::Size;

use chrono::Utc;
use log::info;
use std::fs::create_dir_all;

const OUTPUT_DIR: &str = "img";

pub(crate) fn save_image(buffer: &[u8], size: &Size) {
    ensure_output_dir_exists();

    let file_path = format!("{}/{}", OUTPUT_DIR, get_new_filename());

    info!("Saving image to {}", file_path);
    image::save_buffer(
        file_path,
        buffer,
        size.width,
        size.height,
        image::ColorType::Rgba8,
    )
    .unwrap()
}

fn ensure_output_dir_exists() {
    let _ = create_dir_all(OUTPUT_DIR);
}

fn get_new_filename() -> String {
    format!(
        "{}.png",
        Utc::now()
            .to_string()
            .replace(':', "-")
            .split('.')
            .next()
            .expect("UTC timestamps will contain a decimal for sub-second timing")
    )
}
