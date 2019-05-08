extern crate sdl2;

use std::f32;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;
const WINDOW_PADDING: u32 = 20;

const ISO_PITCH: f32 = (20.0 / 180.0) * f32::consts::PI; // Converted to rads

fn main() -> Result<(), String> {
    let sdl_ctx =sdl2::init()?;
    let vid_subsystem = sdl_ctx.video()?;

    let window =vid_subsystem.window("Business Titan", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 0, 0));

    let h_center = WINDOW_WIDTH / 2;
    // Draw the bottom edge of the grid
    // The edge starts at (h_center, WINDOW_HEIGHT - WINDOW_PADDING)
    let origin = Point::new(h_center as i32, (WINDOW_HEIGHT - WINDOW_PADDING) as i32);
    // I want to fill the screen horizontally so the edge should end at
    let end_x = WINDOW_WIDTH - WINDOW_PADDING;
    let run = end_x - h_center;
    println!("Run: {}", run);
    println!("Tan: {}", ISO_PITCH.tan());
    let rise = (run as f32 * ISO_PITCH.tan()).abs();
    println!("Rise: {}", rise);
    let end_y = WINDOW_HEIGHT - (WINDOW_PADDING + rise as u32);
    let end = Point::new(end_x as i32, end_y as i32);



    // let v_center = WINDOW_HEIGHT as i32 / 2;
    // let p_left = Point::new(20, v_center);
    // let p_top = Point::new(h_center, 20);
    // let p_right = Point::new(WINDOW_WIDTH as i32 - 20, v_center);
    // let p_bottom = Point::new(h_center, WINDOW_HEIGHT as i32 - 20);
    // let lines = [p_left, p_top, p_right, p_bottom, p_left];

    let lines = [origin, end];
    canvas.draw_lines(&lines[..]);
    canvas.present();

    let mut event_pump = sdl_ctx.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
    }

    Ok(())
}
