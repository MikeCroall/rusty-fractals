use crate::mandelbrot_settings::MandelbrotSettings;
use crate::size::Size;
use colors_transform::{Color, Hsl};

pub(crate) fn get_colour_mandelbrot(
    size: &Size,
    x: usize,
    y: usize,
    mbs: &MandelbrotSettings,
) -> [u8; 4] {
    let x0 = mbs.min_x + x as f64 * (mbs.max_x - mbs.min_x) / size.width as f64;
    let y0 = mbs.min_y + y as f64 * (mbs.max_y - mbs.min_y) / size.height as f64;
    let mut iteration = 0;

    // todo: convert to GPU shader? Currently CPU multithreaded rendering managed in main.rs
    let mut x = 0f64;
    let mut y = 0f64;
    let mut x_sq = 0f64;
    let mut y_sq = 0f64;
    while (x_sq + y_sq <= 4f64) && (iteration < mbs.max_iterations) {
        y = 2f64 * x * y + y0;
        x = x_sq - y_sq + x0;
        x_sq = x * x;
        y_sq = y * y;
        iteration += 1;
    }
    get_non_set_colour(iteration, mbs)
}

fn get_non_set_colour(iteration: i32, mbs: &MandelbrotSettings) -> [u8; 4] {
    let hue = 360f32 * iteration as f32 / mbs.max_iterations as f32;
    if iteration == mbs.max_iterations {
        return [0, 0, 0, 255];
    }

    let rgb = Hsl::from(hue, 100.0, 50.0).to_rgb();
    [
        rgb.get_red() as u8,
        rgb.get_green() as u8,
        rgb.get_blue() as u8,
        255,
    ]
}
