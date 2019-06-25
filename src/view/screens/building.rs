use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::Window;

const COLOR_BACKGROUND: (u8, u8, u8) = (132, 132, 123);

pub struct BuyBuildingScreen {
    width: u32,
    height: u32,
    textures: [Texture; 1],
}

impl BuyBuildingScreen {
    pub fn new<T>(
        texture_creator: &TextureCreator<T>,
        font: Font,
        width: u32,
        height: u32,
    ) -> Self {
        let surface = font
            .render("Buy Building")
            .blended(Color::RGB(255, 255, 255))
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        Self {
            width,
            height,
            textures: [texture],
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::from(COLOR_BACKGROUND));
        canvas.fill_rect(Rect::new(0, 0, self.width, self.height))?;

        // Debug display
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_rect(Rect::new(20, 20, self.width - 40, self.height - 40))?;

        let texture = &self.textures[0];
        let TextureQuery { width, height, .. } = texture.query();
        let x = self.width as i32 / 2 - width as i32 / 2;
        canvas.copy(texture, None, Some(Rect::new(x, 40, width, height)))?;

        Ok(())
    }
}
