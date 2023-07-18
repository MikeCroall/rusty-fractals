use crate::mandelbrot_settings::MandelbrotSettings;
use crate::size::Size;
use num::Complex;

#[allow(dead_code)]
pub(crate) fn get_colour_test_gradient(size: &Size, x: usize, y: usize) -> [u8; 4] {
    let r = (255_f32 * x as f32 / size.width as f32) as u8;
    let g = 255 - (255_f32 * x as f32 / size.width as f32) as u8;
    let b = 255 - (255_f32 * y as f32 / size.height as f32) as u8;

    [r, g, b, 255]
}

const NON_SET_COLOURS: [[u8; 4]; 7] = [
    [139, 0, 255, 255],
    [46, 43, 95, 255],
    [0, 0, 255, 255],
    [0, 255, 0, 255],
    [255, 255, 0, 255],
    [255, 127, 0, 255],
    [255, 0, 0, 255],
];

fn get_non_set_colour(iteration: i32, mbs: &MandelbrotSettings) -> [u8; 4] {
    let fractional = iteration as f32 / mbs.max_iterations as f32;
    let index = (fractional * NON_SET_COLOURS.len() as f32) as usize;
    NON_SET_COLOURS[index]
}

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

    if iteration == mbs.max_iterations {
        [0, 0, 0, 255]
    } else {
        get_non_set_colour(iteration, mbs)
    }
}
