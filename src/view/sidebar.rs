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
    button_textures: [Texture; 15],
}

impl Sidebar {
    pub fn new<T>(texture_creator: TextureCreator<T>, width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            button_textures: [
                texture_creator.load_texture("art/close_128.png").unwrap(),
                texture_creator.load_texture("art/save_128.png").unwrap(),
                texture_creator.load_texture("art/music_128.png").unwrap(),
                texture_creator.load_texture("art/eyeball_128.png").unwrap(),
                texture_creator
                    .load_texture("art/question_128.png")
                    .unwrap(),
                texture_creator.load_texture("art/compass_128.png").unwrap(),
                texture_creator.load_texture("art/chart_128.png").unwrap(),
                texture_creator.load_texture("art/cash_128.png").unwrap(),
                texture_creator.load_texture("art/news_128.png").unwrap(),
                texture_creator.load_texture("art/info_128.png").unwrap(),
                texture_creator.load_texture("art/magnify_128.png").unwrap(),
                texture_creator.load_texture("art/factory_128.png").unwrap(),
                texture_creator.load_texture("art/railway_128.png").unwrap(),
                texture_creator
                    .load_texture("art/demolish_128.png")
                    .unwrap(),
                texture_creator.load_texture("art/point_128.png").unwrap(),
            ],
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
        self.draw_button(canvas, Button::Close);
        self.draw_button(canvas, Button::Save);
        self.draw_button(canvas, Button::Music);
        self.draw_button(canvas, Button::Graphics);
        self.draw_button(canvas, Button::Help);

        self.draw_button(canvas, Button::Rotation);
        self.draw_button(canvas, Button::Metrics);
        self.draw_button(canvas, Button::Finances);
        self.draw_button(canvas, Button::News);
        self.draw_button(canvas, Button::Info);

        self.draw_button(canvas, Button::Navigation);
        self.draw_button(canvas, Button::Building);
        self.draw_button(canvas, Button::Rail);
        self.draw_button(canvas, Button::Demolish);
        self.draw_button(canvas, Button::Point);

        Ok(())
    }

    fn draw_button(&self, canvas: &mut Canvas<Window>, button: Button) -> Result<(), String> {
        canvas.copy(
            &self.button_textures[button.texture_index()],
            None,
            Some(Rect::new(
                (button.column() * 32) as i32,
                (button.row() * 32) as i32,
                32 as u32,
                32 as u32,
            )),
        )
    }
}

enum Button {
    Close,
    Save,
    Music,
    Graphics,
    Help,
    Rotation,
    Metrics,
    Finances,
    News,
    Info,
    Navigation,
    Building,
    Rail,
    Demolish,
    Point,
}

impl Button {
    fn texture_index(&self) -> usize {
        match self {
            Button::Close => 0,
            Button::Save => 1,
            Button::Music => 2,
            Button::Graphics => 3,
            Button::Help => 4,

            Button::Rotation => 5,
            Button::Metrics => 6,
            Button::Finances => 7,
            Button::News => 8,
            Button::Info => 9,

            Button::Navigation => 10,
            Button::Building => 11,
            Button::Rail => 12,
            Button::Demolish => 13,
            Button::Point => 14,
        }
    }

    fn row(&self) -> u32 {
        match self {
            Button::Close | Button::Save | Button::Music | Button::Graphics | Button::Help => 0,

            Button::Rotation | Button::Metrics | Button::Finances | Button::News | Button::Info => {
                1
            }

            Button::Navigation
            | Button::Building
            | Button::Rail
            | Button::Demolish
            | Button::Point => 2,
        }
    }

    fn column(&self) -> u32 {
        match self {
            Button::Close => 0,
            Button::Save => 1,
            Button::Music => 2,
            Button::Graphics => 3,
            Button::Help => 4,

            Button::Rotation => 0,
            Button::Metrics => 1,
            Button::Finances => 2,
            Button::News => 3,
            Button::Info => 4,

            Button::Navigation => 0,
            Button::Building => 1,
            Button::Rail => 2,
            Button::Demolish => 3,
            Button::Point => 4,
        }
    }
}
