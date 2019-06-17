extern crate sdl2;

mod state;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use state::{GameBoard, GameState, Vertex, BOARD_HEIGHT, BOARD_WIDTH};
use std::f32::consts::PI;
use std::ops::Sub;
use std::time::{Duration, Instant};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_PADDING: u32 = 20;

const ISO_ANGLE_RADS: f32 = 20.0 / 180.0 * PI;
const ISO_GRID_SIZE: u32 = 100;

const HEIGHT_UNIT_OFFSET: u32 = 5;

const GRID_SCALE: f32 = 20.0;

struct ScreenPoint {
    x: i32,
    y: i32,
}

impl ScreenPoint {
    fn to_render(&self, viewport: &ViewPort) -> rect::Point {
        let viewport_point = viewport.to_viewport_point(&self);

        rect::Point::new(viewport_point.x, viewport_point.y)
    }

    ///  _                _
    /// |                  |
    /// |  cos Θ   -cos Θ  |
    /// |                  |
    /// |  sin Θ    sin Θ  |
    /// |_                _|
    ///
    fn transform(x: f32, y: f32, h: u8) -> (f32, f32) {
        // TODO (toby): Scale these properly.
        let x = x * GRID_SCALE;
        let y = y * GRID_SCALE;

        let out_x = (x - y) * ISO_ANGLE_RADS.cos();
        let mut out_y = (x + y) * ISO_ANGLE_RADS.sin();

        // Apply height transform.
        out_y -= h as f32 * HEIGHT_UNIT_OFFSET as f32;

        (out_x, out_y)
    }
}

impl From<&WorldPoint> for ScreenPoint {
    fn from(world_point: &WorldPoint) -> Self {
        let transformed = Self::transform(world_point.x, world_point.y, world_point.h);

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
    h: u8,
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
            h: 0, // TODO (toby): is it possible to infer actual height?
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
    fn to_viewport_point(&self, screen: &ScreenPoint) -> ViewPortPoint {
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

    fn from_viewport_point(&self, view: &ViewPortPoint) -> ScreenPoint {
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

    let viewport_origin = viewport.to_viewport_point(&ScreenPoint {
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
    board: &GameBoard,
    x: i32,
    y: i32,
    color: Color,
) -> Result<(), String> {
    let prior_color = canvas.draw_color();

    let w_top = x as f32;
    let w_bottom = (x + 1) as f32;
    let w_left = y as f32;
    let w_right = (y + 1) as f32;

    let mut h: u8 = 0;
    if point_on_board(x, y) {
        h = board.vertex_height(Vertex {
            x: x as u32,
            y: y as u32,
        });
    }
    let v_top_left = viewport.to_viewport_point(&ScreenPoint::from(&WorldPoint {
        x: w_top,
        y: w_left,
        h,
    }));
    let mut h: u8 = 0;
    if point_on_board(x, y + 1) {
        h = board.vertex_height(Vertex {
            x: x as u32,
            y: (y + 1) as u32,
        });
    }
    let v_top_right = viewport.to_viewport_point(&ScreenPoint::from(&WorldPoint {
        x: w_top,
        y: w_right,
        h,
    }));
    let mut h: u8 = 0;
    if point_on_board(x + 1, y) {
        h = board.vertex_height(Vertex {
            x: (x + 1) as u32,
            y: y as u32,
        });
    }
    let v_bottom_left = viewport.to_viewport_point(&ScreenPoint::from(&WorldPoint {
        x: w_bottom,
        y: w_left,
        h,
    }));
    let mut h: u8 = 0;
    if point_on_board(x + 1, y + 1) {
        h = board.vertex_height(Vertex {
            x: (x + 1) as u32,
            y: (y + 1) as u32,
        });
    }
    let v_bottom_right = viewport.to_viewport_point(&ScreenPoint::from(&WorldPoint {
        x: w_bottom,
        y: w_right,
        h,
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

fn point_on_board(x: i32, y: i32) -> bool {
    x >= 0 && x <= BOARD_WIDTH as i32 && y >= 0 && y <= BOARD_HEIGHT as i32
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

    let game = GameState::new();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    let viewport_point = &ViewPortPoint { x, y };
                    let screen_point = viewport.from_viewport_point(&viewport_point);
                    let world_point: WorldPoint = (&screen_point).into();

                    cur_block_x = world_point.x as i32;
                    if world_point.x < 0.0 {
                        cur_block_x = (world_point.x - 1.0) as i32;
                    }
                    cur_block_y = world_point.y as i32;
                    if world_point.y < 0.0 {
                        cur_block_y = (world_point.y - 1.0) as i32;
                    }
                }
                Event::MouseButtonDown { x, y, .. } => {
                    println!("Click! View: ({}, {})", x, y);
                    let viewport_point = &ViewPortPoint { x, y };

                    let screen_point = viewport.from_viewport_point(&viewport_point);
                    println!("Screen: ({}, {})", screen_point.x, screen_point.y);

                    let world_point: WorldPoint = (&screen_point).into();
                    println!("World: ({}, {})", world_point.x, world_point.y);

                    // Update viewport focal point
                    // viewport.focal_point = screen_point;

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

        canvas.set_draw_color(Color::RGB(53, 117, 189));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        draw_iso_sprite(
            &mut canvas,
            &viewport,
            &tx,
            WorldPoint {
                x: 5.0,
                y: 5.0,
                h: 0,
            },
        )?;

        let color = Color::RGBA(255, 255, 255, 150);
        fill_block(
            &mut canvas,
            &viewport,
            &game.board,
            cur_block_x,
            cur_block_y,
            color,
        )?;

        let mut draw_lines = Vec::new();
        for world_point in &lines {
            draw_lines.push(ScreenPoint::from(world_point).to_render(&viewport));
        }

        canvas.draw_lines(draw_lines.as_slice())?;

        // Get the bounding corners of the view port in terms of world points.
        // This will be a irregular quadralateral (possibly trapezoid?) within the game world.
        let view_top_left_screen: ScreenPoint =
            viewport.from_viewport_point(&ViewPortPoint { x: 0, y: 0 });
        let view_top_left_world: WorldPoint = (&view_top_left_screen).into();
        let view_top_right_screen: ScreenPoint = viewport.from_viewport_point(&ViewPortPoint {
            x: WINDOW_WIDTH as i32,
            y: 0,
        });
        let view_top_right_world: WorldPoint = (&view_top_right_screen).into();
        let view_bottom_left_screen: ScreenPoint = viewport.from_viewport_point(&ViewPortPoint {
            x: 0,
            y: WINDOW_HEIGHT as i32,
        });
        let view_bottom_left_world: WorldPoint = (&view_bottom_left_screen).into();
        let view_bottom_right_screen: ScreenPoint = viewport.from_viewport_point(&ViewPortPoint {
            x: WINDOW_WIDTH as i32,
            y: WINDOW_HEIGHT as i32,
        });
        let view_bottom_right_world: WorldPoint = (&view_bottom_right_screen).into();

        // Rather than any complex math, we can just draw a recangular bounding box around our region.
        // This will capture more region than necessary but is fast and guarantees our viewport is a subregion.
        let x_vals = [
            view_top_left_world.x,
            view_top_right_world.x,
            view_bottom_left_world.x,
            view_bottom_right_world.x,
        ];
        let y_vals = [
            view_top_left_world.y,
            view_top_right_world.y,
            view_bottom_left_world.y,
            view_bottom_right_world.y,
        ];
        let mut min_x = view_top_left_world.x;
        let mut max_x = view_top_left_world.x;
        let mut min_y = view_top_left_world.y;
        let mut max_y = view_top_left_world.y;
        for x in x_vals.iter() {
            if *x < min_x {
                min_x = *x;
            }
            if *x > max_x {
                max_x = *x;
            }
        }
        for y in y_vals.iter() {
            if *y < min_y {
                min_y = *y;
            }
            if *y > max_y {
                max_y = *y;
            }
        }

        let draw_begin = Instant::now();
        for y in 0..(BOARD_HEIGHT + 1) {
            // Skip any points outside viewport bounding box.
            if (y as f32) < min_y || (y as f32) > max_y {
                continue;
            }

            for x in 0..(BOARD_WIDTH + 1) {
                // Skip any points outside viewport bounding box.
                if (x as f32) < min_x || (x as f32) > max_x {
                    continue;
                }

                // Map the board vertex to a ViewportPoint and draw the point.
                let h = game.board.vertex_height(Vertex { x, y });
                let x = x as f32;
                let y = y as f32;

                let point = WorldPoint { x, y, h };

                let view_point = ScreenPoint::from(&point).to_render(&viewport);
                canvas.draw_point(view_point)?;
            }
        }

        // println!("Compute and draw: {:?}", draw_begin.elapsed());
        canvas.present();
    }

    Ok(())
}
