extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::f32::consts::PI;
use std::ops::Sub;
use std::time::{Duration, Instant};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_PADDING: u32 = 20;

const ISO_ANGLE_RADS: f32 = 20.0 / 180.0 * PI;
const ISO_GRID_SIZE: u32 = 100;

const GRID_SCALE: f32 = 20.0;

struct ScreenPoint {
    x: i32,
    y: i32,
}

impl ScreenPoint {
    fn to_render(&self) -> rect::Point {
        rect::Point::new(self.x, self.y)
    }

    ///  _                _
    /// |                  |
    /// |  cos Θ   -cos Θ  |
    /// |                  |
    /// |  sin Θ    sin Θ  |
    /// |_                _|
    ///
    fn transform(x: f32, y: f32) -> (f32, f32) {
        // TODO (toby): Scale these properly.
        let x = x * GRID_SCALE;
        let y = y * GRID_SCALE;

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

/// Represents a point on the game world surface.
/// The surface is a continuous plane, hence the use of floating points for locations.
/// Within this surface, however, is a unit grid system.
/// The units correspond to integer values, for example (1.0, 3.0).
/// Most world objects fill NxM units, for some integers N and M, and are positioned directly on grid edges.
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

        // TODO (toby): Scale these properly.
        (out_x / GRID_SCALE, out_y / GRID_SCALE)
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

fn draw_iso_sprite(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    origin: WorldPoint,
) -> Result<(), String> {
    // Translate the origin point to a screen point.
    let screen = ScreenPoint::from(&origin);

    let scale = 0.1;

    // Find the appropriate offset of the top-left of the sprite.
    let q = texture.query();
    let scaled_w = q.width as f32 * scale;
    let scaled_h = q.height as f32 * scale;

    // println!("Texture: w: {}; h: {}", q.width, q.height);
    // Anchor is center-bottom of texture.
    let anchor_x = scaled_w / 2.0;
    let anchor_y = scaled_h;
    let screen_offset_x = screen.x - anchor_x as i32;
    let screen_offset_y = screen.y - anchor_y as i32;
    // println!("Offset X: {}; Y: {}", screen_offset_x, screen_offset_y);

    // Draw the sprite.
    canvas.copy(
        &texture,
        None,
        Some(rect::Rect::new(
            screen_offset_x,
            screen_offset_y,
            scaled_w as u32,
            scaled_h as u32,
        )),
    )?;

    Ok(())
}

fn fill_block(canvas: &mut Canvas<Window>, x: u32, y: u32) -> Result<(), String> {
    let w_top = x as f32;
    let w_bottom = (x + 1) as f32;
    let w_left = y as f32;
    let w_right = (y + 1) as f32;

    let s_top_left = ScreenPoint::from(&WorldPoint {
        x: w_top,
        y: w_left,
    });
    let s_top_right = ScreenPoint::from(&WorldPoint {
        x: w_top,
        y: w_right,
    });
    let s_bottom_left = ScreenPoint::from(&WorldPoint {
        x: w_bottom,
        y: w_left,
    });
    let s_bottom_right = ScreenPoint::from(&WorldPoint {
        x: w_bottom,
        y: w_right,
    });

    let vx = [
        s_top_left.x as i16,
        s_top_right.x as i16,
        s_bottom_right.x as i16,
        s_bottom_left.x as i16,
    ];
    let vy = [
        s_top_left.y as i16,
        s_top_right.y as i16,
        s_bottom_right.y as i16,
        s_bottom_left.y as i16,
    ];

    let color = Color::RGBA(255, 255, 255, 150);
    canvas.filled_polygon(&vx[..], &vy[..], color)?;

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init()?;
    let vid_subsystem = sdl_ctx.video()?;

    let window = vid_subsystem
        .window("Titan", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_ctx.event_pump()?;
    let mut lines = Vec::<WorldPoint>::new();

    let tx = texture_creator.load_texture("art/test_sprite.png")?;

    let mut cur_block_x = 0;
    let mut cur_block_y = 0;
    let mut scale = 1.0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { x, y, .. } => {
                    println!("Click!");
                    let screen_point = &ScreenPoint { x, y };
                    println!("Screen: ({}, {})", screen_point.x, screen_point.y);
                    let world_point: WorldPoint = screen_point.into();
                    println!("World: ({}, {})", world_point.x, world_point.y);

                    cur_block_x = world_point.x as u32;
                    cur_block_y = world_point.y as u32;

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
                }
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        let draw_begin = Instant::now();
        canvas.set_draw_color(Color::RGB(53, 117, 189));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        for i in 0..(ISO_GRID_SIZE as i32 + 1) {
            let start = WorldPoint {
                x: 0.0,
                y: i as f32,
            };
            let end = WorldPoint {
                x: ISO_GRID_SIZE as f32,
                y: i as f32,
            };

            canvas.draw_line(
                ScreenPoint::from(&start).to_render(),
                ScreenPoint::from(&end).to_render(),
            )?;
        }

        for i in 0..(ISO_GRID_SIZE as i32 + 1) {
            let start = WorldPoint {
                x: i as f32,
                y: 0.0,
            };
            let end = WorldPoint {
                x: i as f32,
                y: ISO_GRID_SIZE as f32,
            };

            canvas.draw_line(
                ScreenPoint::from(&start).to_render(),
                ScreenPoint::from(&end).to_render(),
            )?;
        }

        let mut draw_lines = Vec::new();
        for world_point in &lines {
            draw_lines.push(ScreenPoint::from(world_point).to_render());
        }
        // println!("{:?}", draw_lines);
        canvas.draw_lines(draw_lines.as_slice())?;

        draw_iso_sprite(&mut canvas, &tx, WorldPoint { x: 5.0, y: 5.0 })?;

        fill_block(&mut canvas, cur_block_x, cur_block_y)?;

        canvas.present();
        let draw_time = draw_begin.elapsed();
    }

    Ok(())
}
