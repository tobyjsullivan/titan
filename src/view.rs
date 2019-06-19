use crate::action::GameAction;
use crate::controller::PlayerAction;
use crate::state::{
    Block, GameBoard, GameState, LandType, PlayerMode, SelectionMode, Vertex, WATER_LEVEL,
};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f32::consts::PI;
use std::time::Instant;

const ISO_ANGLE_RADS: f32 = 20.0 / 180.0 * PI;

const HEIGHT_UNIT_OFFSET: u32 = 5;

const GRID_SCALE: f32 = 20.0;

const COLOR_WHITE: (u8, u8, u8) = (255, 255, 255);
const COLOR_DARK_GRAY: (u8, u8, u8) = (37, 37, 37);
const COLOR_BLACK: (u8, u8, u8) = (0, 0, 0);
const COLOR_HIGHLIGHT_BLOCK: (u8, u8, u8, u8) = (255, 255, 255, 150);
const COLOR_WATER: (u8, u8, u8) = (53, 117, 189);
const COLOR_LAND: (u8, u8, u8) = (0, 200, 0);

#[derive(Debug)]
pub struct ScreenPoint {
    x: i32,
    y: i32,
}

impl ScreenPoint {
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
#[derive(Debug)]
pub struct WorldPoint {
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

#[derive(Debug)]
pub struct ViewportPoint {
    pub x: i32,
    pub y: i32,
}

impl Into<rect::Point> for ViewportPoint {
    fn into(self) -> rect::Point {
        rect::Point::new(self.x, self.y)
    }
}

pub struct Viewport {
    window_width: u32,
    window_height: u32,
    // TODO (toby): scale: f32,
    // TODO (toby): rotation
}

impl Viewport {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            window_width: width,
            window_height: height,
        }
    }

    fn compute_viewport_point(&self, game: &GameState, world_point: WorldPoint) -> ViewportPoint {
        let screen_point = ScreenPoint::from(&world_point);
        let screen_focal_point = ScreenPoint::from(&WorldPoint {
            x: game.focal_point.x as f32,
            y: game.focal_point.y as f32,
            // Should height be normalised to keep scrolling smooth?
            h: game.board.vertex_height(game.focal_point),
        });

        let vertex_offset_x = screen_point.x - screen_focal_point.x;
        let vertex_offset_y = screen_point.y - screen_focal_point.y;

        let viewport_center_x = self.window_width / 2;
        let viewport_center_y = self.window_height / 2;

        ViewportPoint {
            x: viewport_center_x as i32 + vertex_offset_x,
            y: viewport_center_y as i32 + vertex_offset_y,
        }
    }

    fn compute_world_point(&self, game: &GameState, viewport_point: ViewportPoint) -> WorldPoint {
        // Take the mouse position and compute its offset from the centre of the screen.
        let viewport_center_x = self.window_width / 2;
        let viewport_center_y = self.window_height / 2;
        let screen_offset_x = viewport_point.x - viewport_center_x as i32;
        let screen_offset_y = viewport_point.y - viewport_center_y as i32;

        // Compute the current focal screen point.
        let screen_focal_point = ScreenPoint::from(&WorldPoint {
            x: game.focal_point.x as f32,
            y: game.focal_point.y as f32,
            // Should height be normalised to keep scrolling smooth?
            h: game.board.vertex_height(game.focal_point),
        });

        // Add the offset to the focal screen point to the mouse viewport position to get the
        // mouse position screen point.
        let point_screen_x = screen_focal_point.x + screen_offset_x;
        let point_screen_y = screen_focal_point.y + screen_offset_y;

        // Apply the inverted linear mapping (including scaling) to this mouse position screen
        // point to get the mouse position world point.
        WorldPoint::from(&ScreenPoint {
            x: point_screen_x,
            y: point_screen_y,
        })
    }

    pub fn get_block_under_cursor(&self, game: &GameState, x: i32, y: i32) -> Option<Block> {
        let world_point = self.compute_world_point(game, ViewportPoint { x, y });

        let mut cur_block_x = world_point.x as i32;
        let mut cur_block_y = world_point.y as i32;
        if world_point.x < 0.0 {
            None
        } else if world_point.y < 0.0 {
            None
        } else {
            Some(Block {
                x: cur_block_x as u32,
                y: cur_block_y as u32,
            })
        }
    }

    pub fn map_player_action(
        &self,
        game: &GameState,
        player_action: PlayerAction,
    ) -> Option<GameAction> {
        match (&game.player_mode, &player_action) {
            (PlayerMode::Focus, PlayerAction::WindowLeftClick { .. }) => Some(GameAction::Focus),
            (PlayerMode::RaiseLower { .. }, PlayerAction::WindowLeftClick { .. }) => {
                Some(GameAction::RaiseTerrain)
            }
            (PlayerMode::RaiseLower { .. }, PlayerAction::WindowRightClick { .. }) => {
                Some(GameAction::LowerTerrain)
            }
            _ => None,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, game: &GameState) -> Result<(), String> {
        canvas.set_draw_color(COLOR_DARK_GRAY);
        canvas.clear();

        canvas.set_draw_color(COLOR_BLACK);

        // Get the bounding corners of the view port in terms of world points.
        // This will be a irregular quadralateral (possibly trapezoid?) within the game world.
        let view_top_left_world: WorldPoint =
            self.compute_world_point(game, ViewportPoint { x: 0, y: 0 });
        let view_top_right_world: WorldPoint = self.compute_world_point(
            game,
            ViewportPoint {
                x: self.window_width as i32,
                y: 0,
            },
        );
        let view_bottom_left_world: WorldPoint = self.compute_world_point(
            game,
            ViewportPoint {
                x: 0,
                y: self.window_height as i32,
            },
        );
        let view_bottom_right_world: WorldPoint = self.compute_world_point(
            game,
            ViewportPoint {
                x: self.window_width as i32,
                y: self.window_height as i32,
            },
        );

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
        for &x in x_vals.iter() {
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
        }
        for &y in y_vals.iter() {
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }

        let draw_begin = Instant::now();
        let board_width = game.board.width();
        let board_height = game.board.height();
        for y in 0..(board_height + 1) {
            // Skip any points outside viewport bounding box.
            if (y as f32) < min_y || (y as f32) > max_y {
                continue;
            }

            for x in 0..(board_width + 1) {
                // Skip any points outside viewport bounding box.
                if (x as f32) < min_x || (x as f32) > max_x {
                    continue;
                }

                if x < board_width && y < board_height {
                    let mut tile_color = match game.board.block_land_type(x, y) {
                        LandType::Water => Color::from(COLOR_WATER),
                        LandType::Land => Color::from(COLOR_LAND),
                    };

                    fill_block(canvas, &self, &game, x as i32, y as i32, tile_color)?;
                }

                // Map the board vertex to a ViewportPoint and draw the point.
                let h = game.board.vertex_height(Vertex { x, y });
                let x = x as f32;
                let y = y as f32;

                let world_point = WorldPoint { x, y, h };
                let viewport_point = self.compute_viewport_point(game, world_point);

                let point: rect::Point = viewport_point.into();
                canvas.draw_point(point)?;
            }
        }

        // Highlight the block currently under the cursor.
        if let Some(block) = &game.highlighted_block {
            match game.selection_mode() {
                SelectionMode::None => {}
                SelectionMode::Blocks { w, h } => {
                    // TODO (toby): Highlight all blocks in (w X h) with current block at the top.
                    // w is positive along x-axis; h is positive along y-axis.
                    for y in 0..h {
                        for x in 0..w {
                            fill_block(
                                canvas,
                                &self,
                                &game,
                                block.x as i32 + x as i32,
                                block.y as i32 + y as i32,
                                Color::from(COLOR_HIGHLIGHT_BLOCK),
                            )?;
                        }
                    }
                }
                SelectionMode::Vertex { radius } => {
                    for y in (block.y as i32 - radius as i32)..(block.y as i32 + radius as i32 + 1)
                    {
                        for x in
                            (block.x as i32 - radius as i32)..(block.x as i32 + radius as i32 + 1)
                        {
                            draw_vertex(canvas, &self, &game, x, y, Color::from(COLOR_WHITE));
                        }
                    }
                }
            }
        }

        // println!("Compute and draw: {:?}", draw_begin.elapsed());

        Ok(())
    }
}

fn draw_vertex(
    canvas: &mut Canvas<Window>,
    viewport: &Viewport,
    game: &GameState,
    x: i32,
    y: i32,
    color: Color,
) -> Result<(), String> {
    let prior_color = canvas.draw_color();

    let w_top = x as f32;
    let w_left = y as f32;

    let mut h: u8 = 0;
    if x >= 0
        && y >= 0
        && game.board.vertex_on_board(Vertex {
            x: x as u32,
            y: y as u32,
        })
    {
        h = game.board.vertex_height(Vertex {
            x: x as u32,
            y: y as u32,
        });
    }
    let v_top_left = viewport.compute_viewport_point(
        game,
        WorldPoint {
            x: w_top,
            y: w_left,
            h,
        },
    );

    canvas.set_draw_color(color);
    canvas.draw_point(rect::Point::new(v_top_left.x, v_top_left.y));

    canvas.set_draw_color(prior_color);

    Ok(())
}

fn fill_block(
    canvas: &mut Canvas<Window>,
    viewport: &Viewport,
    game: &GameState,
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
    if x >= 0
        && y >= 0
        && game.board.vertex_on_board(Vertex {
            x: x as u32,
            y: y as u32,
        })
    {
        h = game.board.vertex_height(Vertex {
            x: x as u32,
            y: y as u32,
        });
    }
    let v_top_left = viewport.compute_viewport_point(
        game,
        WorldPoint {
            x: w_top,
            y: w_left,
            h,
        },
    );
    let mut h: u8 = 0;
    if x >= 0
        && y + 1 >= 0
        && game.board.vertex_on_board(Vertex {
            x: x as u32,
            y: (y + 1) as u32,
        })
    {
        h = game.board.vertex_height(Vertex {
            x: x as u32,
            y: (y + 1) as u32,
        });
    }
    let v_top_right = viewport.compute_viewport_point(
        game,
        WorldPoint {
            x: w_top,
            y: w_right,
            h,
        },
    );
    let mut h: u8 = 0;
    if x + 1 >= 0
        && y >= 0
        && game.board.vertex_on_board(Vertex {
            x: (x + 1) as u32,
            y: y as u32,
        })
    {
        h = game.board.vertex_height(Vertex {
            x: (x + 1) as u32,
            y: y as u32,
        });
    }
    let v_bottom_left = viewport.compute_viewport_point(
        game,
        WorldPoint {
            x: w_bottom,
            y: w_left,
            h,
        },
    );
    let mut h: u8 = 0;
    if x + 1 >= 0
        && y + 1 >= 0
        && game.board.vertex_on_board(Vertex {
            x: (x + 1) as u32,
            y: (y + 1) as u32,
        })
    {
        h = game.board.vertex_height(Vertex {
            x: (x + 1) as u32,
            y: (y + 1) as u32,
        });
    }
    let v_bottom_right = viewport.compute_viewport_point(
        game,
        WorldPoint {
            x: w_bottom,
            y: w_right,
            h,
        },
    );

    let lines = vec![
        rect::Point::new(v_top_left.x, v_top_left.y),
        rect::Point::new(v_top_right.x, v_top_right.y),
        rect::Point::new(v_bottom_right.x, v_bottom_right.y),
        rect::Point::new(v_bottom_left.x, v_bottom_left.y),
        rect::Point::new(v_top_left.x, v_top_left.y),
    ];

    canvas.set_draw_color(color);
    canvas.draw_lines(lines.as_slice())?;

    // Filled polygons can be restored when a better play experience is needed.
    // let vx = [
    //     v_top_left.x as i16,
    //     v_top_right.x as i16,
    //     v_bottom_right.x as i16,
    //     v_bottom_left.x as i16,
    // ];
    // let vy = [
    //     v_top_left.y as i16,
    //     v_top_right.y as i16,
    //     v_bottom_right.y as i16,
    //     v_bottom_left.y as i16,
    // ];

    // canvas.filled_polygon(&vx[..], &vy[..], color)?;

    canvas.set_draw_color(prior_color);

    Ok(())
}
