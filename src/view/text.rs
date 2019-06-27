use sdl2::image::LoadTexture;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, RenderTarget, Texture, TextureCreator};

const SOURCE_TEXT_HEIGHT: u32 = 52;

pub struct DynamicText {
    text_height: u32,
    texture: Texture,
}

impl DynamicText {
    pub fn new<T>(texture_creator: &TextureCreator<T>, text_height: u32) -> Self {
        Self {
            text_height,
            texture: texture_creator.load_texture("art/font_52.png").unwrap(),
        }
    }

    pub fn print<T: RenderTarget>(&self, canvas: &mut Canvas<T>, dst: Point) -> Result<(), String> {
        let text_scale = self.text_height as f32 / SOURCE_TEXT_HEIGHT as f32;
        let sample = "Sample Text";

        let mut offset: u32 = 0;
        for c in sample.chars() {
            let y = Self::char_index(c) * SOURCE_TEXT_HEIGHT;
            let width = Self::char_width(c);
            let src = Rect::new(0, y as i32, SOURCE_TEXT_HEIGHT, width);
            let scaled_width = (width as f32 * text_scale) as u32;
            let dst = Rect::new(dst.x + offset as i32, dst.y, self.text_height, scaled_width);
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

    fn char_width(c: char) -> u32 {
        match c {
            // TODO (toby)
            _ => SOURCE_TEXT_HEIGHT,
        }
    }
}
