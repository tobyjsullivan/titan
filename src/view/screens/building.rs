use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator, TextureQuery};
use sdl2::video::Window;

const COLOR_BACKGROUND: (u8, u8, u8) = (82, 82, 82);

pub struct BuyBuildingScreen {
    width: u32,
    height: u32,
    dialog_width: u32,
    dialog_height: u32,
    textures: [Texture; 1],
}

impl BuyBuildingScreen {
    pub fn new<T>(
        texture_creator: &TextureCreator<T>,
        width: u32,
        height: u32,
        dialog_width: u32,
        dialog_height: u32,
    ) -> Self {
        let texture = texture_creator
            .load_texture("art/buy_building_2560.png")
            .unwrap();
        Self {
            width,
            height,
            dialog_width,
            dialog_height,
            textures: [texture],
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::from(COLOR_BACKGROUND));
        canvas.fill_rect(Rect::new(0, 0, self.width, self.height))?;

        let dialog_left = (self.width - self.dialog_width) as i32 / 2;
        let dialog_top = (self.height - self.dialog_height) as i32 / 2;

        let texture = &self.textures[0];
        canvas.copy(texture, None, Some(Rect::new(dialog_left, dialog_top, self.dialog_width, self.dialog_height)))?;

        // Debug display
        // canvas.set_draw_color(Color::RGB(255, 0, 0));
        // canvas.draw_rect(Rect::new(dialog_left, dialog_top, self.dialog_width, self.dialog_height))?;

        Ok(())
    }
}
