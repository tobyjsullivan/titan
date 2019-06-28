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
    inverted_texture: Texture,
}

impl DynamicText {
    pub fn new<T>(texture_creator: &TextureCreator<T>, screen: ScreenState) -> Self {
        let texture = Self::load_texture(texture_creator);
        let mut inverted_texture = Self::load_texture(texture_creator);
        inverted_texture.set_color_mod(0, 0, 0);

        Self {
            screen,
            texture,
            inverted_texture,
        }
    }

    pub fn print<T: RenderTarget>(
        &self,
        canvas: &mut Canvas<T>,
        content: &str,
        dst: Point,
        invert: bool,
    ) -> Result<(), String> {
        let text_height = self.screen.scale_y(TEXT_HEIGHT as i32) as u32;
        let text_scale = text_height as f32 / SOURCE_TEXT_HEIGHT as f32;

        let mut offset: u32 = 0;
        for c in content.chars() {
            let y = Self::char_index(c) * SOURCE_TEXT_HEIGHT;
            let width = Self::source_char_width(c);
            let src = Rect::new(0, y as i32, width, SOURCE_TEXT_HEIGHT);
            let scaled_width = (width as f32 * text_scale) as u32;
            let dst = Rect::new(dst.x + offset as i32, dst.y, scaled_width, text_height);
            offset += scaled_width;
            if invert {
                canvas.copy(&self.inverted_texture, src, dst)?;
            } else {
                canvas.copy(&self.texture, src, dst)?;
            }
        }

        Ok(())
    }

    fn load_texture<T>(texture_creator: &TextureCreator<T>) -> Texture {
        let mut surface: Surface = Surface::from_file("art/font_52.png").unwrap();
        surface
            .set_color_key(true, Color::from((0, 0, 0)))
            .expect("failed to set color key");

        texture_creator
            .create_texture_from_surface(surface)
            .expect("failed to convert surface to texture")
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
            'A' => 30,
            'B' => 30,
            'C' => 30,
            'D' => 30,
            'E' => 25,
            'F' => 25,
            'G' => 30,
            'H' => 30,
            'I' => 15,
            'J' => 30,
            'K' => 34,
            'L' => 25,
            'M' => 50,
            'N' => 33,
            'O' => 30,
            'P' => 30,
            'Q' => 30,
            'R' => 30,
            'S' => 25,
            'T' => 30,
            'U' => 30,
            'V' => 30,
            'W' => 50,
            'X' => 30,
            'Y' => 30,
            'Z' => 30,
            'a' => 30,
            'b' => 30,
            'c' => 25,
            'd' => 30,
            'e' => 30,
            'f' => 25,
            'g' => 30,
            'h' => 30,
            'i' => 12,
            'j' => 25,
            'k' => 33,
            'l' => 12,
            'm' => 50,
            'n' => 30,
            'o' => 30,
            'p' => 30,
            'q' => 30,
            'r' => 25,
            's' => 25,
            't' => 20,
            'u' => 30,
            'v' => 30,
            'w' => 45,
            'x' => 30,
            'y' => 30,
            'z' => 30,
            '0' => 30,
            '1' => 18,
            '2' => 30,
            '3' => 30,
            '4' => 33,
            '5' => 30,
            '6' => 30,
            '7' => 30,
            '8' => 30,
            '9' => 30,
            '!' => 12,
            '"' => 18,
            '$' => 25,
            '%' => 43,
            '&' => 40,
            '\'' => 8,
            '(' => 15,
            ')' => 15,
            '*' => 25,
            '+' => 25,
            ',' => 12,
            '-' => 25,
            '.' => 12,
            '/' => 25,
            ':' => 12,
            ';' => 12,
            '<' => 25,
            '=' => 30,
            '>' => 25,
            '?' => 30,
            '@' => 43,
            '[' => 18,
            '\\' => 25,
            ']' => 18,
            '_' => 33,
            ' ' => 30,
            _ => 30,
        }
    }
}
