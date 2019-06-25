extern crate sdl2;

mod action;
mod state;
mod systems;
mod view;

use action::GameAction;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use state::{GameState, PlayerMode};
use std::ops::{Add, Sub};
use std::thread;
use std::time::{Duration, Instant};
use view::{Interface, KeyboardKey, PlayerInteraction};

const TEXT_HEIGHT: u32 = 20;

const SIDEBAR_WIDTH: u32 = 160;

const UPDATES_PER_SECOND: u32 = 120;
const MAX_FRAMES_PER_SECOND: u32 = 60;

fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init()?;
    let vid_subsystem = sdl_ctx.video()?;

    let window = vid_subsystem
        .window("Titan", 800, 600)
        .position_centered()
        .allow_highdpi()
        .fullscreen_desktop()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let (window_width, window_height) = window.size();
    let (drawable_x, drawable_y) = window.drawable_size();
    let scale_x = drawable_x / window_width;
    let scale_y = drawable_y / window_height;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_ctx.event_pump()?;

    let interface = Interface::new(
        texture_creator,
        drawable_x,
        drawable_y,
        TEXT_HEIGHT * scale_y,
        SIDEBAR_WIDTH * scale_x,
    );

    let mut game = GameState::new();

    let update_interval = Duration::new(0, 1_000_000_000 / UPDATES_PER_SECOND);
    let mut next_update = Instant::now();
    let mut last_frame = Instant::now();
    let mut frame_count: u64 = 0;
    'running: loop {
        let mut player_interactions = Vec::new();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    let key = match keycode {
                        Keycode::Space => Some(KeyboardKey::Space),
                        _ => None,
                    };
                    if let Some(key) = key {
                        player_interactions.push(PlayerInteraction::KeyPress { key });
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    let x = x * scale_x as i32;
                    let y = y * scale_y as i32;
                    let player_action = PlayerInteraction::CursorMove { x, y };
                    player_interactions.push(player_action);
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    let x = x * scale_x as i32;
                    let y = y * scale_y as i32;
                    let player_action = match mouse_btn {
                        MouseButton::Left => Some(PlayerInteraction::WindowLeftClick { x, y }),
                        MouseButton::Right => Some(PlayerInteraction::WindowRightClick { x, y }),
                        _ => None,
                    };
                    if let Some(player_action) = player_action {
                        player_interactions.push(player_action);
                    }
                }
                _ => {}
            }
        }

        // Resolve all actions.
        for &player_interaction in player_interactions.iter() {
            match interface.map_player_interaction(&game, player_interaction) {
                Some(GameAction::Hover { block }) => {
                    systems::navigation::apply_hover(&mut game, block)
                }
                Some(GameAction::Focus) => {
                    systems::navigation::apply_focus(&mut game);
                }
                Some(GameAction::LowerTerrain) => {
                    systems::terrain::apply_lower_terrain(&mut game);
                }
                Some(GameAction::OpenMenu { menu }) => {
                    systems::menu::apply_open_menu(&mut game, menu);
                }
                Some(GameAction::PlaceStructure) => {
                    systems::structure::apply_place_structure(&mut game);
                }
                Some(GameAction::RaiseTerrain) => {
                    systems::terrain::apply_raise_terrain(&mut game);
                }
                Some(GameAction::RotateStructure) => {
                    systems::structure::apply_rotate_structure(&mut game);
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
            interface.render(&mut canvas, &game)?;
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
