extern crate sdl2;

mod state;
mod view;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use state::{GameState, PlayerMode};
use view::{ViewPort, ViewPortPoint};

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

    let mut viewport = ViewPort::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut game = GameState::new();
    'running: loop {
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
                Event::MouseButtonDown { x, y, .. } => {
                    let viewport_point = ViewPortPoint { x, y };
                    println!("Click! View: {:?}", viewport_point);

                    match game.player_mode {
                        PlayerMode::Focus => viewport.update_focus(viewport_point),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        viewport.render(&mut canvas, &game)?;
        canvas.present();
    }

    Ok(())
}
