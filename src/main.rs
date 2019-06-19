extern crate sdl2;

mod action;
mod controller;
mod state;
mod systems;
mod view;

use action::GameAction;
use controller::PlayerAction;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use state::{GameState, PlayerMode};
use view::{Viewport, ViewportPoint};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

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

    let mut event_pump = sdl_ctx.event_pump()?;

    let mut viewport = Viewport::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut game = GameState::new();
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
                    game.highlighted_block = viewport.get_block_under_cursor(x, y);
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    let viewport_point = ViewportPoint { x, y };
                    println!("Click! View: {:?}", viewport_point);

                    let player_action = match mouse_btn {
                        MouseButton::Left => Some(PlayerAction::WindowLeftClick { x, y }),
                        MouseButton::Right => Some(PlayerAction::WindowRightClick { x, y }),
                        _ => None,
                    };
                    if let Some(player_action) = player_action {
                        player_actions.push(player_action);
                    }

                    match game.player_mode {
                        PlayerMode::Focus => viewport.update_focus(viewport_point),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Resolve all actions.
        for &player_action in player_actions.iter() {
            // TODO (toby): Utilize this result.
            match controller::map_player_action(&viewport, &game, player_action) {
                Some(game_action @ GameAction::LowerTerrain) => {
                    systems::terrain::apply(&mut game, game_action)
                }
                Some(game_action @ GameAction::RaiseTerrain) => {
                    systems::terrain::apply(&mut game, game_action)
                }
                None => {}
            }
        }

        viewport.render(&mut canvas, &game)?;
        canvas.present();
    }

    Ok(())
}
