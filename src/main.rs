#![forbid(unsafe_code)]

mod io;
mod mandelbrot_settings;
mod pixel_colour;
mod size;

use error_iter::ErrorIter as _;
use log::{error, info};
use mandelbrot_settings::MandelbrotSettings;
use pixels::{Error, Pixels, SurfaceTexture};
use rayon::prelude::*;
use size::Size;
use std::time::Instant;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::{io::save_image, pixel_colour::get_colour_mandelbrot};

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut render_size = Size::default();
    let mut mandelbrot_settings = MandelbrotSettings::default();

    // let monitor = event_loop.available_monitors().next().expect("No monitor!"); // todo toggleable fullscreen
    let window = {
        let size = LogicalSize::new(render_size.width as f64, render_size.height as f64);
        WindowBuilder::new()
            .with_title("Rusty Fractals")
            .with_inner_size(size)
            // .with_fullscreen(Some(winit::window::Fullscreen::Borderless(Some(
            //     monitor.clone(),
            // ))))
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height, surface_texture)?
    };

    let threads: usize = std::thread::available_parallelism().unwrap().into();
    info!("Found available parallelism of {}", threads);

    let mut paused = true;
    event_loop.run(move |event, _, control_flow| {
        // non-winit_input_helper events
        if let Event::RedrawRequested(_) = event {
            draw_cpu_multithreaded(&mandelbrot_settings, &render_size, &mut pixels, threads);
            mandelbrot_settings.notify_rendered();
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // winit_input_helper events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Pause
            if input.key_pressed(VirtualKeyCode::P) {
                paused = !paused;
            }

            // Step frame by frame so ensure paused
            if input.key_pressed_os(VirtualKeyCode::Space) {
                paused = true;
            }

            // Save image file of current render
            if input.key_pressed(VirtualKeyCode::S) {
                save_image(pixels.frame(), &render_size);
            }

            // Reset pan, zoom, and iterations
            if input.key_pressed(VirtualKeyCode::R) {
                mandelbrot_settings.pan_and_zoom_reset();
                mandelbrot_settings.iterations_reset();
            }

            // Pan
            if input.key_pressed_os(VirtualKeyCode::Left) {
                mandelbrot_settings.pan_left();
            } else if input.key_pressed_os(VirtualKeyCode::Right) {
                mandelbrot_settings.pan_right();
            }
            if input.key_pressed_os(VirtualKeyCode::Up) {
                mandelbrot_settings.pan_up();
            } else if input.key_pressed_os(VirtualKeyCode::Down) {
                mandelbrot_settings.pan_down();
            }

            // Zoom
            if input.key_pressed_os(VirtualKeyCode::Z) {
                mandelbrot_settings.zoom_in();
            } else if input.key_pressed_os(VirtualKeyCode::X) {
                mandelbrot_settings.zoom_out();
            }

            // Scroll iterations
            let scroll = input.scroll_diff();
            if scroll != 0f32 {
                mandelbrot_settings.add_iterations(scroll as i32);
                info!(
                    "New max iterations: {} (scrolled by {})",
                    mandelbrot_settings.max_iterations,
                    format!("{:>+1}", scroll)
                );
            }

            // Resize the window // fixme window resizing is broken since adding rayon multithreaded rendering
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                if size.width != render_size.width || size.height != render_size.height {
                    pixels
                        .resize_buffer(size.width, size.height)
                        .expect("Failed to resize buffer");
                    render_size.width = size.width;
                    render_size.height = size.height;
                }
            }

            // Re-draw if not paused, if settings changed, or if pressing the step frame-by-frame key (space)
            if !paused
                || mandelbrot_settings.needs_re_render()
                || input.key_pressed_os(VirtualKeyCode::Space)
            {
                window.request_redraw();
            }
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

fn draw_cpu_multithreaded(
    mandelbrot_settings: &MandelbrotSettings,
    size: &Size,
    pixels: &mut Pixels,
    threads: usize,
) {
    let start_time = Instant::now();
    let len = pixels.frame_mut().len();
    let chunk_size = len / threads;

    pixels
        .frame_mut()
        .par_chunks_mut(chunk_size)
        .enumerate()
        .for_each(|(chunk_index, chunk)| {
            chunk
                .chunks_mut(4)
                .enumerate()
                .for_each(|(pixel_index, pixel)| {
                    let window_pixel_index = (chunk_index * chunk_size / 4) + pixel_index;
                    let x = window_pixel_index % size.width as usize;
                    let y = window_pixel_index / size.width as usize;
                    let colour = get_colour_mandelbrot(size, x, y, mandelbrot_settings);
                    pixel.copy_from_slice(&colour);
                });
        });
    let elapsed = start_time.elapsed().as_millis();
    info!(target: "draw", "Rendering for {size} took {elapsed}ms");
}
