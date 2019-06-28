use crate::view::text::DynamicText;
use crate::view::{ScreenState, DIALOG_HEIGHT, DIALOG_WIDTH};
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator, TextureQuery};
use sdl2::video::Window;
use std::rc::Rc;

const COLOR_BACKGROUND: (u8, u8, u8) = (82, 82, 82);

pub struct BuyBuildingScreen {
    screen: ScreenState,
    dynamic_text: Rc<DynamicText>,
    textures: [Texture; 1],
}

impl BuyBuildingScreen {
    pub fn new<T>(
        texture_creator: &TextureCreator<T>,
        dynamic_text: Rc<DynamicText>,
        screen: ScreenState,
    ) -> Self {
        let texture = texture_creator
            .load_texture("art/buy_building_2560.png")
            .unwrap();
        Self {
            dynamic_text,
            screen,
            textures: [texture],
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let (screen_width, screen_height) = self.screen.size();

        canvas.set_draw_color(Color::from(COLOR_BACKGROUND));
        let screen_rect = Rect::new(0, 0, screen_width, screen_height);
        canvas.fill_rect(screen_rect)?;

        let dialog_width = self.screen.scale_x(DIALOG_WIDTH as i32) as u32;
        let dialog_height = self.screen.scale_y(DIALOG_HEIGHT as i32) as u32;

        let screen_center = screen_rect.center();
        let dialog_rect = Rect::from_center(screen_center, dialog_width, dialog_height);

        let texture = &self.textures[0];
        canvas.copy(texture, None, Some(dialog_rect))?;

        // Debug display
        // canvas.set_draw_color(Color::RGB(255, 0, 0));
        // canvas.draw_rect(Rect::new(dialog_left, dialog_top, self.dialog_width, self.dialog_height))?;
        self.dynamic_text.print(
            canvas,
            "Hello, world!",
            Point::new(
                dialog_rect.left() + self.screen.scale_x(30),
                dialog_rect.top() + self.screen.scale_y(30),
            ),
        )?;

        Ok(())
    }
}
