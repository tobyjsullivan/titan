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
    fn to_render(&self, viewport: &ViewPort) -> rect::Point {
        let viewport_point = viewport.to_view_port(&self);

        rect::Point::new(viewport_point.x, viewport_point.y)
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
        let transformed = Self::transform(world_point.x, world_point.y);

        ScreenPoint {
            x: transformed.0 as i32,
            y: transformed.1 as i32,
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
        // Apply the inverse transform to get world point.
        let inverse = Self::inverse_transform(screen_point.x as f32, screen_point.y as f32);

        Self {
            x: inverse.0,
            y: inverse.1,
        }
    }
}

struct ViewPortPoint {
    x: i32,
    y: i32,
}

struct ViewPort {
    focal_point: ScreenPoint,
    // TODO (toby): scale: f32,
    // TODO (toby): rotation
}

impl ViewPort {
    fn to_view_port(&self, screen: &ScreenPoint) -> ViewPortPoint {
        // Adjust coordinates by viewport offset.
        let x: i32 = screen.x - self.focal_point.x;
        let y: i32 = screen.y - self.focal_point.y;

        // Translate point to centre of screen.
        let h_center = WINDOW_WIDTH / 2;
        let v_center = WINDOW_HEIGHT / 2;
        let x = x + h_center as i32;
        let y = y + v_center as i32;

        ViewPortPoint { x, y }
    }

    fn from_view_port(&self, view: &ViewPortPoint) -> ScreenPoint {
        // Translate point from centre of screen.
        let h_center = WINDOW_WIDTH / 2;
        let v_center = WINDOW_HEIGHT / 2;
        let x = view.x - h_center as i32;
        let y = view.y - v_center as i32;

        // Adjust coordinates by viewport offset.
        let x = x + self.focal_point.x;
        let y = y + self.focal_point.y;

        ScreenPoint { x, y }
    }
}

fn draw_iso_sprite(
    canvas: &mut Canvas<Window>,
    viewport: &ViewPort,
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

    // Anchor is center-bottom of texture.
    let anchor_x = scaled_w / 2.0;
    let anchor_y = scaled_h;
    let screen_offset_x = screen.x - anchor_x as i32;
    let screen_offset_y = screen.y - anchor_y as i32;

    let viewport_origin = viewport.to_view_port(&ScreenPoint {
        x: screen_offset_x,
        y: screen_offset_y,
    });

    // Draw the sprite.
    canvas.copy(
        &texture,
        None,
        Some(rect::Rect::new(
            viewport_origin.x,
            viewport_origin.y,
            scaled_w as u32,
            scaled_h as u32,
        )),
    )?;

    Ok(())
}

fn fill_block(
    canvas: &mut Canvas<Window>,
    viewport: &ViewPort,
    x: i32,
    y: i32,
    color: Color,
) -> Result<(), String> {
    let prior_color = canvas.draw_color();

    let w_top = x as f32;
    let w_bottom = (x + 1) as f32;
    let w_left = y as f32;
    let w_right = (y + 1) as f32;

    let v_top_left = viewport.to_view_port(&ScreenPoint::from(&WorldPoint {
        x: w_top,
        y: w_left,
    }));
    let v_top_right = viewport.to_view_port(&ScreenPoint::from(&WorldPoint {
        x: w_top,
        y: w_right,
    }));
    let v_bottom_left = viewport.to_view_port(&ScreenPoint::from(&WorldPoint {
        x: w_bottom,
        y: w_left,
    }));
    let v_bottom_right = viewport.to_view_port(&ScreenPoint::from(&WorldPoint {
        x: w_bottom,
        y: w_right,
    }));

    let vx = [
        v_top_left.x as i16,
        v_top_right.x as i16,
        v_bottom_right.x as i16,
        v_bottom_left.x as i16,
    ];
    let vy = [
        v_top_left.y as i16,
        v_top_right.y as i16,
        v_bottom_right.y as i16,
        v_bottom_left.y as i16,
    ];

    canvas.filled_polygon(&vx[..], &vy[..], color)?;

    canvas.set_draw_color(prior_color);

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

    let mut viewport = ViewPort {
        focal_point: ScreenPoint { x: 100, y: 200 },
    };
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
                    println!("Click! View: ({}, {})", x, y);
                    let viewport_point = &ViewPortPoint { x, y };

                    let screen_point = viewport.from_view_port(&viewport_point);
                    println!("Screen: ({}, {})", screen_point.x, screen_point.y);

                    let world_point: WorldPoint = (&screen_point).into();
                    println!("World: ({}, {})", world_point.x, world_point.y);

                    // Update viewport focal point
                    // viewport.focal_point = screen_point;

                    cur_block_x = world_point.x as i32;
                    if world_point.x < 0.0 {
                        cur_block_x = (world_point.x - 1.0) as i32;
                    }
                    cur_block_y = world_point.y as i32;
                    if world_point.y < 0.0 {
                        cur_block_y = (world_point.y - 1.0) as i32;
                    }

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
                ScreenPoint::from(&start).to_render(&viewport),
                ScreenPoint::from(&end).to_render(&viewport),
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
                ScreenPoint::from(&start).to_render(&viewport),
                ScreenPoint::from(&end).to_render(&viewport),
            )?;
        }

        draw_iso_sprite(&mut canvas, &viewport, &tx, WorldPoint { x: 5.0, y: 5.0 })?;

        let color = Color::RGBA(255, 255, 255, 150);
        fill_block(&mut canvas, &viewport, cur_block_x, cur_block_y, color)?;

        let mut draw_lines = Vec::new();
        for world_point in &lines {
            draw_lines.push(ScreenPoint::from(world_point).to_render(&viewport));
        }

        canvas.draw_lines(draw_lines.as_slice())?;

        canvas.present();
        let draw_time = draw_begin.elapsed();
    }

    Ok(())
}
