use super::{ScreenState, TEXT_HEIGHT};
use sdl2::image::{LoadSurface, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, RenderTarget, Texture, TextureCreator};
use sdl2::surface::Surface;

const SOURCE_TEXT_HEIGHT: u32 = 52;

pub struct DynamicText {
    screen: ScreenState,
    texture: Texture,
}

impl DynamicText {
    pub fn new<T>(texture_creator: &TextureCreator<T>, screen: ScreenState) -> Self {
        let mut surface: Surface = Surface::from_file("art/font_52.png").unwrap();
        surface
            .set_color_key(true, Color::from((0, 0, 0)))
            .expect("failed to set color key");
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .expect("failed to convert surface to texture");

        Self { screen, texture }
    }

    pub fn print<T: RenderTarget>(
        &self,
        canvas: &mut Canvas<T>,
        content: &str,
        dst: Point,
    ) -> Result<(), String> {
        let text_height = self.screen.scale_y(TEXT_HEIGHT as i32) as u32;
        let text_scale = text_height as f32 / SOURCE_TEXT_HEIGHT as f32;

        let mut offset: u32 = 0;
        for c in content.chars() {
            let y = Self::char_index(c) * SOURCE_TEXT_HEIGHT;
            let width = Self::source_char_width(c);
            let src = Rect::new(0, y as i32, SOURCE_TEXT_HEIGHT, width);
            let scaled_width = (width as f32 * text_scale) as u32;
            let dst = Rect::new(dst.x + offset as i32, dst.y, text_height, scaled_width);
            offset += scaled_width;
            canvas.copy(&self.texture, src, dst)?;
        }

        Ok(())
    }

    fn char_index(c: char) -> u32 {
        match c {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            'H' => 7,
            'I' => 8,
            'J' => 9,
            'K' => 10,
            'L' => 11,
            'M' => 12,
            'N' => 13,
            'O' => 14,
            'P' => 15,
            'Q' => 16,
            'R' => 17,
            'S' => 18,
            'T' => 19,
            'U' => 20,
            'V' => 21,
            'W' => 22,
            'X' => 23,
            'Y' => 24,
            'Z' => 25,
            'a' => 26,
            'b' => 27,
            'c' => 28,
            'd' => 29,
            'e' => 30,
            'f' => 31,
            'g' => 32,
            'h' => 33,
            'i' => 34,
            'j' => 35,
            'k' => 36,
            'l' => 37,
            'm' => 38,
            'n' => 39,
            'o' => 40,
            'p' => 41,
            'q' => 42,
            'r' => 43,
            's' => 44,
            't' => 45,
            'u' => 46,
            'v' => 47,
            'w' => 48,
            'x' => 49,
            'y' => 50,
            'z' => 51,
            '0' => 52,
            '1' => 53,
            '2' => 54,
            '3' => 55,
            '4' => 56,
            '5' => 57,
            '6' => 58,
            '7' => 59,
            '8' => 60,
            '9' => 61,
            '!' => 62,
            '"' => 63,
            '$' => 64,
            '%' => 65,
            '&' => 66,
            '\'' => 67,
            '(' => 68,
            ')' => 69,
            '*' => 70,
            '+' => 71,
            ',' => 72,
            '-' => 73,
            '.' => 74,
            '/' => 75,
            ':' => 76,
            ';' => 77,
            '<' => 78,
            '=' => 79,
            '>' => 80,
            '?' => 81,
            '@' => 82,
            '[' => 83,
            '\\' => 84,
            ']' => 85,
            '_' => 86,
            ' ' => 87,
            _ => 81,
        }
    }

    fn source_char_width(c: char) -> u32 {
        match c {
            // TODO (toby)
            _ => SOURCE_TEXT_HEIGHT,
        }
    }
}
