use crate::action::GameAction;
use crate::controller::PlayerAction;
use crate::state::GameState;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::Window;

const COLOR_SIDEBAR: (u8, u8, u8) = (132, 132, 123);

pub struct Sidebar {
    width: u32,
    height: u32,
    btn_close: Texture,
    btn_save: Texture,
    btn_music: Texture,
    btn_graphics: Texture,
    btn_help: Texture,
    btn_rotate: Texture,
    btn_graphs: Texture,
    btn_money: Texture,
    btn_news: Texture,
    btn_info: Texture,
    btn_navigate: Texture,
    btn_building: Texture,
    btn_rail: Texture,
    btn_demolish: Texture,
    btn_point: Texture,
}

impl Sidebar {
    pub fn new<T>(texture_creator: TextureCreator<T>, width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            btn_close: texture_creator.load_texture("art/close_128.png").unwrap(),
            btn_save: texture_creator.load_texture("art/save_128.png").unwrap(),
            btn_music: texture_creator.load_texture("art/music_128.png").unwrap(),
            btn_graphics: texture_creator.load_texture("art/eyeball_128.png").unwrap(),
            btn_help: texture_creator
                .load_texture("art/question_128.png")
                .unwrap(),

            btn_rotate: texture_creator.load_texture("art/compass_128.png").unwrap(),
            btn_graphs: texture_creator.load_texture("art/chart_128.png").unwrap(),
            btn_money: texture_creator.load_texture("art/cash_128.png").unwrap(),
            btn_news: texture_creator.load_texture("art/news_128.png").unwrap(),
            btn_info: texture_creator.load_texture("art/info_128.png").unwrap(),

            btn_navigate: texture_creator.load_texture("art/magnify_128.png").unwrap(),
            btn_building: texture_creator.load_texture("art/factory_128.png").unwrap(),
            btn_rail: texture_creator.load_texture("art/railway_128.png").unwrap(),
            btn_demolish: texture_creator
                .load_texture("art/demolish_128.png")
                .unwrap(),
            btn_point: texture_creator.load_texture("art/point_128.png").unwrap(),
        }
    }

    pub fn map_player_action(
        &self,
        game: &GameState,
        player_action: PlayerAction,
    ) -> Option<GameAction> {
        match &player_action {
            // TODO (toby)
            _ => None,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, game: &GameState) -> Result<(), String> {
        canvas.set_draw_color(Color::from(COLOR_SIDEBAR));
        canvas.fill_rect(Rect::new(0, 0, self.width, self.height))?;

        // Draw buttons
        Self::draw_button(canvas, 0, 0, &self.btn_close)?;
        Self::draw_button(canvas, 0, 1, &self.btn_save)?;
        Self::draw_button(canvas, 0, 2, &self.btn_music)?;
        Self::draw_button(canvas, 0, 3, &self.btn_graphics)?;
        Self::draw_button(canvas, 0, 4, &self.btn_help)?;

        Self::draw_button(canvas, 1, 0, &self.btn_rotate)?;
        Self::draw_button(canvas, 1, 1, &self.btn_graphs)?;
        Self::draw_button(canvas, 1, 2, &self.btn_money)?;
        Self::draw_button(canvas, 1, 3, &self.btn_news)?;
        Self::draw_button(canvas, 1, 4, &self.btn_info)?;

        Self::draw_button(canvas, 2, 0, &self.btn_navigate)?;
        Self::draw_button(canvas, 2, 1, &self.btn_building)?;
        Self::draw_button(canvas, 2, 2, &self.btn_rail)?;
        Self::draw_button(canvas, 2, 3, &self.btn_demolish)?;
        Self::draw_button(canvas, 2, 4, &self.btn_point)?;

        Ok(())
    }

    fn draw_button(
        canvas: &mut Canvas<Window>,
        row: u32,
        column: u32,
        texture: &Texture,
    ) -> Result<(), String> {
        canvas.copy(
            texture,
            None,
            Some(Rect::new(
                (column * 32) as i32,
                (row * 32) as i32,
                32 as u32,
                32 as u32,
            )),
        )
    }
}
