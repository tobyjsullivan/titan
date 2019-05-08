extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect;
use std::f32::consts::PI;
use std::ops::Sub;
use std::time::{Duration, Instant};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_PADDING: u32 = 20;

const ISO_ANGLE_RADS: f32 = 20.0 / 180.0 * PI;
const ISO_GRID_SIZE: u32 = 100;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn transform(&self, x: i32, y: i32) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }

    fn to_render(&self, scale: f32) -> rect::Point {
        let h_center = WINDOW_WIDTH as i32 / 2;
        let v_center = WINDOW_HEIGHT as i32 / 2;

        rect::Point::new(h_center + (self.x as f32 * scale) as i32, v_center + (self.y as f32 * scale) as i32)
    }
}

fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init()?;
    let vid_subsystem = sdl_ctx.video()?;

    let window = vid_subsystem.window("Titan", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_ctx.event_pump()?;

    let mut scale = 1.0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { x, y, .. } => {
                    register_mouse_down(x, y, scale);
                }
                Event::MouseWheel { y, .. } => {
                    scale += (y as f32) / 10.0;
                    if scale < 0.10 {
                        scale = 0.10
                    }

                    if scale > 10.0 {
                        scale = 10.0
                    }
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        let draw_begin = Instant::now();
        canvas.set_draw_color(Color::RGB(53, 117, 189));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        for i in 0..(ISO_GRID_SIZE as i32 + 1) {
            let offset = i - (ISO_GRID_SIZE as i32 / 2);

            let start = transform(-5 * (ISO_GRID_SIZE as i32), 10 * offset);
            let end = transform(5 * (ISO_GRID_SIZE as i32), 10 * offset);

            canvas.draw_line(start.to_render(scale), end.to_render(scale));
        }

        for i in 0..(ISO_GRID_SIZE as i32 + 1) {
            let offset = i - (ISO_GRID_SIZE as i32 / 2);

            let start = transform(10 * offset, -5 * (ISO_GRID_SIZE as i32));
            let end = transform(10 * offset, 5 * (ISO_GRID_SIZE as i32));

            canvas.draw_line(start.to_render(scale), end.to_render(scale));
        }

        canvas.present();
        let draw_time = draw_begin.elapsed();
    }

    Ok(())
}

fn transform(x: i32, y: i32) -> Point {
    let out_x = (x - y) as f32 * ISO_ANGLE_RADS.cos();
    let out_y = (x + y) as f32 * ISO_ANGLE_RADS.sin();

    Point { x: out_x as i32, y: out_y as i32 }
}

fn register_mouse_down(x: i32, y: i32, scale: f32) {
    // TODO
}
