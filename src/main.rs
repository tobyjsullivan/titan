extern crate sdl2;

mod action;
mod controller;
mod state;
mod systems;
mod view;

use action::GameAction;
use controller::{PlayerAction, WindowPanel};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use state::{GameState, PlayerMode};
use std::ops::{Add, Sub};
use std::thread;
use std::time::{Duration, Instant};
use view::sidebar::Sidebar;
use view::viewport::Viewport;
use view::COLOR_DARK_GRAY;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

const SIDEBAR_WIDTH: u32 = 160;

const UPDATES_PER_SECOND: u32 = 120;
const MAX_FRAMES_PER_SECOND: u32 = 60;

fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init()?;
    let vid_subsystem = sdl_ctx.video()?;

    let window = vid_subsystem
        .window("Titan", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .allow_highdpi()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let (drawable_x, drawable_y) = window.drawable_size();
    let scale_x = drawable_x / WINDOW_WIDTH;
    let scale_y = drawable_y / WINDOW_HEIGHT;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_ctx.event_pump()?;

    let mut viewport = Viewport::new(
        WINDOW_WIDTH * scale_x - SIDEBAR_WIDTH * scale_x,
        WINDOW_HEIGHT * scale_y,
        SIDEBAR_WIDTH * scale_x,
    );
    let mut sidebar = Sidebar::new(
        texture_creator,
        SIDEBAR_WIDTH * scale_x,
        WINDOW_HEIGHT * scale_y,
        scale_x,
        scale_y,
    );
    let mut game = GameState::new();

    let update_interval = Duration::new(0, 1_000_000_000 / UPDATES_PER_SECOND);
    let mut next_update = Instant::now();
    let mut last_frame = Instant::now();
    let mut frame_count: u64 = 0;
    'running: loop {
        let mut player_actions = Vec::new();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    let player_action = PlayerAction::CursorMove {
                        panel: window_panel(x, y),
                        x: x * scale_x as i32,
                        y: y * scale_y as i32,
                    };
                    player_actions.push(player_action);
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    let player_action = match mouse_btn {
                        MouseButton::Left => Some(PlayerAction::WindowLeftClick {
                            panel: window_panel(x, y),
                            x: x * scale_x as i32,
                            y: y * scale_y as i32,
                        }),
                        MouseButton::Right => Some(PlayerAction::WindowRightClick {
                            panel: window_panel(x, y),
                            x: x * scale_x as i32,
                            y: y * scale_y as i32,
                        }),
                        _ => None,
                    };
                    if let Some(player_action) = player_action {
                        player_actions.push(player_action);
                    }
                }
                _ => {}
            }
        }

        // Resolve all actions.
        for &player_action in player_actions.iter() {
            match controller::map_player_action(&sidebar, &viewport, &game, player_action) {
                Some(GameAction::Hover { block }) => {
                    systems::navigation::apply_hover(&mut game, block)
                }
                Some(GameAction::Focus) => {
                    systems::navigation::apply_focus(&mut game);
                }
                Some(GameAction::LowerTerrain) => {
                    systems::terrain::apply_lower_terrain(&mut game);
                }
                Some(GameAction::RaiseTerrain) => {
                    systems::terrain::apply_raise_terrain(&mut game);
                }
                Some(GameAction::SidebarHover { button }) => {
                    systems::navigation::apply_sidebar_hover(&mut game, button);
                }
                None => {}
            }
        }

        next_update = next_update.add(update_interval);

        if last_frame.elapsed() > Duration::new(0, 1_000_000_000 / MAX_FRAMES_PER_SECOND) {
            let render_start = Instant::now();
            canvas.set_draw_color(COLOR_DARK_GRAY);
            canvas.clear();

            viewport.render(&mut canvas, &game)?;
            sidebar.render(&mut canvas, &game)?;
            canvas.present();

            frame_count += 1;
            // println!("Frame {}: {:?}", frame_count, render_start.elapsed());
            last_frame = Instant::now();
        }

        let now = Instant::now();
        if next_update > now {
            let delay = next_update.sub(now);
            thread::sleep(delay);
        }
    }

    Ok(())
}

fn window_panel(x: i32, y: i32) -> WindowPanel {
    if x <= SIDEBAR_WIDTH as i32 {
        WindowPanel::Sidebar
    } else {
        WindowPanel::Viewport
    }
}
