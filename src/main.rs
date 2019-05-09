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

struct ScreenPoint {
    x: i32,
    y: i32,
}

impl ScreenPoint {
    fn to_render(&self) -> rect::Point {
        rect::Point::new(self.x, self.y)
    }

    ///  _               _
    /// |                 |
    /// | cos Θ   -cos Θ  |
    /// |                 |
    /// | sin Θ    sin Θ  |
    /// |_               _|
    ///
    fn transform(x: f32, y: f32) -> (f32, f32) {
        let out_x = (x - y) * ISO_ANGLE_RADS.cos();
        let out_y = (x + y) * ISO_ANGLE_RADS.sin();

        (out_x, out_y)
    }
}

impl From<&WorldPoint> for ScreenPoint {
    fn from(world_point: &WorldPoint) -> Self {
        // First apply transform to point.
        let transformed = Self::transform(world_point.x, world_point.y);

        // Finally, translate value to viewport.
        let h_center = WINDOW_WIDTH as f32 / 2.0;
        let v_center = WINDOW_HEIGHT as f32 / 2.0;

        ScreenPoint {
            x: (transformed.0 + h_center) as i32,
            y: (transformed.1 + v_center) as i32,
        }
    }
}

struct WorldPoint {
    x: f32,
    y: f32,
}

impl WorldPoint {
    ///  _                         _
    /// |                           |
    /// | 1/(2cos Θ)    1/(2sin Θ)  |
    /// |                           |
    /// | -1/(2cos Θ)   1/(2sin Θ)  |
    /// |_                         _|
    ///
    fn inverse_transform(x: f32, y: f32) -> (f32, f32) {
        let out_x = x / (2.0 * ISO_ANGLE_RADS.cos()) + y / (2.0 * ISO_ANGLE_RADS.sin());
        let out_y = -x / (2.0 * ISO_ANGLE_RADS.cos()) + y / (2.0 * ISO_ANGLE_RADS.sin());

        (out_x, out_y)
    }
}

impl From<&ScreenPoint> for WorldPoint {
    fn from(screen_point: &ScreenPoint) -> Self {
        // First, translate to viewport.
        let h_center = WINDOW_WIDTH as f32 / 2.0;
        let v_center = WINDOW_HEIGHT as f32 / 2.0;

        let x = screen_point.x as f32 - h_center;
        let y = screen_point.y as f32 - v_center;

        // Apply the inverse transform to get world point.
        let inverse = Self::inverse_transform(x, y);

        Self {
            x: inverse.0,
            y: inverse.1,
        }
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
    let mut lines = Vec::<WorldPoint>::new();

    let mut scale = 1.0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { x, y, .. } => {
                    let screen_point = &ScreenPoint { x, y };
                    let world_point: WorldPoint = screen_point.into();
                    lines.push(world_point);
                }
                Event::MouseWheel { y, .. } => {
                    // TODO (toby): Restore scaling
                    // scale += (y as f32) / 10.0;
                    // if scale < 0.10 {
                    //     scale = 0.10
                    // }

                    // if scale > 10.0 {
                    //     scale = 10.0
                    // }
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

            let start = WorldPoint { x: (-5 * ISO_GRID_SIZE as i32) as f32, y: (10 * offset) as f32 };
            let end = WorldPoint { x: (5 * ISO_GRID_SIZE as i32) as f32, y: (10 * offset) as f32 };

            canvas.draw_line(ScreenPoint::from(&start).to_render(), ScreenPoint::from(&end).to_render());
        }

        for i in 0..(ISO_GRID_SIZE as i32 + 1) {
            let offset = i - (ISO_GRID_SIZE as i32 / 2);

            let start = WorldPoint { x: (10 * offset) as f32, y: (-5 * ISO_GRID_SIZE as i32) as f32 };
            let end = WorldPoint { x: (10 * offset) as f32, y: (5 * ISO_GRID_SIZE as i32) as f32 };

            canvas.draw_line(ScreenPoint::from(&start).to_render(), ScreenPoint::from(&end).to_render());
        }

        let mut draw_lines = Vec::new();
        for world_point in &lines {
            draw_lines.push(ScreenPoint::from(world_point).to_render());
        }
        // println!("{:?}", draw_lines);
        canvas.draw_lines(draw_lines.as_slice());

        canvas.present();
        let draw_time = draw_begin.elapsed();
    }

    Ok(())
}
