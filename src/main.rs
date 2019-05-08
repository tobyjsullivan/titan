extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect;
use std::f32::consts::PI;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;
const WINDOW_PADDING: u32 = 20;

const ISO_ANGLE_RADS: f32 = 20.0 / 180.0 * PI;
const ISO_GRID_SIZE: u32 = 20;

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

    fn to_render(&self) -> rect::Point {
        let h_center = WINDOW_WIDTH as i32 / 2;
        let v_center = WINDOW_HEIGHT as i32 / 2;

        rect::Point::new(h_center + self.x, v_center + self.y)
    }
}

fn main() -> Result<(), String> {
    let sdl_ctx =sdl2::init()?;
    let vid_subsystem = sdl_ctx.video()?;

    let window =vid_subsystem.window("Titan", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 0, 0));

    for i in -10..11 {
        let start = transform(-100, 10 * i);
        let end = transform(100, 10 * i);

        canvas.draw_line(start.to_render(), end.to_render());
    }

    for i in -10..11 {
        let start = transform(10 * i, -100);
        let end = transform(10 * i, 100);

        canvas.draw_line(start.to_render(), end.to_render());
    }

    canvas.present();

    let mut event_pump = sdl_ctx.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
    }

    Ok(())
}

fn transform(x: i32, y: i32) -> Point {
    let out_x = (x - y) as f32 * ISO_ANGLE_RADS.cos();
    let out_y = (x + y) as f32 * ISO_ANGLE_RADS.sin();

    Point { x: out_x as i32, y: out_y as i32 }
}
