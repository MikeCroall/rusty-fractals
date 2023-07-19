#![forbid(unsafe_code)]

mod mandelbrot_settings;
mod pixel_colour;
mod size;

use error_iter::ErrorIter as _;
use log::{error, info};
use mandelbrot_settings::MandelbrotSettings;
use pixels::{Error, Pixels, SurfaceTexture};
use size::Size;
use std::time::Instant;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::pixel_colour::get_colour_mandelbrot;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut render_size = Size::default();
    let mut mandelbrot_settings = MandelbrotSettings::default();

    // let monitor = event_loop.available_monitors().next().expect("No monitor!");
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

    let mut paused = true;
    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            draw(&mandelbrot_settings, &render_size, pixels.frame_mut());
            mandelbrot_settings.notify_rendered();
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.
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

            // Reset pan and zoom
            if input.key_pressed(VirtualKeyCode::R) {
                mandelbrot_settings.pan_reset();
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

            // Resize the window
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

    #[allow(unreachable_code)] // unreachable return but shows complaint if no return...
    Ok(())
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

fn draw(mandelbrot_settings: &MandelbrotSettings, size: &Size, screen: &mut [u8]) {
    let start_time = Instant::now();
    for x in 0..size.width as usize {
        for y in 0..size.height as usize {
            let pixel_index: usize = x * 4 + y * size.width as usize * 4;
            let colour: [u8; 4] = get_colour_mandelbrot(size, x, y, mandelbrot_settings);
            screen[pixel_index..pixel_index + colour.len()].copy_from_slice(&colour);
        }
    }
    let elapsed = start_time.elapsed().as_millis();
    info!(target: "draw", "Rendering for {size} took {elapsed}ms");
}
