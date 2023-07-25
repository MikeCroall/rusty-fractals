use crate::mandelbrot_settings::MandelbrotSettings;
use crate::size::Size;
use colors_transform::{Color, Hsl};
use num::Complex;

pub(crate) fn get_colour_mandelbrot(
    size: &Size,
    x: usize,
    y: usize,
    mbs: &MandelbrotSettings,
) -> [u8; 4] {
    let real = mbs.min_x + x as f64 * (mbs.max_x - mbs.min_x) / size.width as f64;
    let imag = mbs.min_y + y as f64 * (mbs.max_y - mbs.min_y) / size.height as f64;

    let c = Complex::new(real, imag);
    let mut z = Complex::new(0.0, 0.0);

    let mut iteration = 0;
    while iteration < mbs.max_iterations && z.norm() <= 2.0 {
        z = z * z + c;
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
