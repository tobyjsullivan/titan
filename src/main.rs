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
use view::sidebar::Sidebar;
use view::viewport::Viewport;
use view::COLOR_DARK_GRAY;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

const SIDEBAR_WIDTH: u32 = 200;

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

    let mut viewport = Viewport::new(WINDOW_WIDTH - SIDEBAR_WIDTH, WINDOW_HEIGHT, SIDEBAR_WIDTH);
    let mut sidebar = Sidebar::new(SIDEBAR_WIDTH, WINDOW_HEIGHT);

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
                    let player_action = PlayerAction::CursorMove {
                        panel: window_panel(x, y),
                        x,
                        y,
                    };
                    player_actions.push(player_action);
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    let player_action = match mouse_btn {
                        MouseButton::Left => Some(PlayerAction::WindowLeftClick {
                            panel: window_panel(x, y),
                            x,
                            y,
                        }),
                        MouseButton::Right => Some(PlayerAction::WindowRightClick {
                            panel: window_panel(x, y),
                            x,
                            y,
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
                None => {}
            }
        }

        canvas.set_draw_color(COLOR_DARK_GRAY);
        canvas.clear();

        viewport.render(&mut canvas, &game)?;
        sidebar.render(&mut canvas, &game)?;
        canvas.present();
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
