use crate::action::GameAction;
use crate::state::{Direction, GameState, PlayerMode, SidebarMenu};
use crate::view::{PlayerAction, WindowPanel};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::Window;

const COLOR_SIDEBAR: (u8, u8, u8) = (132, 132, 123);
const COLOR_DEPRESSED_BUTTON_OUTLINE: (u8, u8, u8) = (255, 0, 0);
const COLOR_DEPRESSED_BUTTON_BACKGROUND: (u8, u8, u8, u8) = (255, 255, 255, 100);

const BUTTONS_PER_ROW: u32 = 5;

pub struct Sidebar {
    width: u32,
    height: u32,
    text_height: u32,
    button_textures: [Texture; 15],
}

impl Sidebar {
    pub fn new<T>(
        texture_creator: TextureCreator<T>,
        width: u32,
        height: u32,
        text_height: u32,
    ) -> Self {
        Self {
            width,
            height,
            text_height,
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
                texture_creator.load_texture("art/dozer_128.png").unwrap(),
                texture_creator.load_texture("art/point_128.png").unwrap(),
            ],
        }
    }

    pub fn cursor_move_action(&self, x: i32, y: i32) -> Option<GameAction> {
        Some(GameAction::SidebarHover {
            button: self.button_under_cursor(x, y),
        })
    }

    pub fn left_click_action(&self, x: i32, y: i32) -> Option<GameAction> {
        match self.button_under_cursor(x, y) {
            Some(menu) => Some(GameAction::OpenMenu { menu }),
            None => None,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, game: &GameState) -> Result<(), String> {
        canvas.set_draw_color(Color::from(COLOR_SIDEBAR));
        canvas.fill_rect(Rect::new(0, 0, self.width, self.height))?;

        // Draw buttons
        self.draw_button(canvas, game, SidebarMenu::Close)?;
        self.draw_button(canvas, game, SidebarMenu::Save)?;
        self.draw_button(canvas, game, SidebarMenu::Music)?;
        self.draw_button(canvas, game, SidebarMenu::Graphics)?;
        self.draw_button(canvas, game, SidebarMenu::Help)?;

        self.draw_button(canvas, game, SidebarMenu::Rotation)?;
        self.draw_button(canvas, game, SidebarMenu::Metrics)?;
        self.draw_button(canvas, game, SidebarMenu::Finances)?;
        self.draw_button(canvas, game, SidebarMenu::News)?;
        self.draw_button(canvas, game, SidebarMenu::Info)?;

        self.draw_button(canvas, game, SidebarMenu::Navigation)?;
        self.draw_button(canvas, game, SidebarMenu::Building)?;
        self.draw_button(canvas, game, SidebarMenu::Rail)?;
        self.draw_button(canvas, game, SidebarMenu::Demolish)?;
        self.draw_button(canvas, game, SidebarMenu::Point)?;

        Ok(())
    }

    fn button_under_cursor(&self, x: i32, y: i32) -> Option<SidebarMenu> {
        let button_width = self.width / BUTTONS_PER_ROW;
        let button_height = button_width; // Square buttons
        let button_grid_offset_y = self.text_height * 3;

        match (
            x / button_width as i32,
            (y - button_grid_offset_y as i32) / button_height as i32,
        ) {
            (0, 0) => Some(SidebarMenu::Close),
            (1, 0) => Some(SidebarMenu::Save),
            (2, 0) => Some(SidebarMenu::Music),
            (3, 0) => Some(SidebarMenu::Graphics),
            (4, 0) => Some(SidebarMenu::Help),

            (0, 1) => Some(SidebarMenu::Rotation),
            (1, 1) => Some(SidebarMenu::Metrics),
            (2, 1) => Some(SidebarMenu::Finances),
            (3, 1) => Some(SidebarMenu::News),
            (4, 1) => Some(SidebarMenu::Info),

            (0, 2) => Some(SidebarMenu::Navigation),
            (1, 2) => Some(SidebarMenu::Building),
            (2, 2) => Some(SidebarMenu::Rail),
            (3, 2) => Some(SidebarMenu::Demolish),
            (4, 2) => Some(SidebarMenu::Point),

            (_, _) => None,
        }
    }

    fn draw_button(
        &self,
        canvas: &mut Canvas<Window>,
        game: &GameState,
        button: SidebarMenu,
    ) -> Result<(), String> {
        let button_width = self.width / BUTTONS_PER_ROW;
        let button_height = button_width; // Square buttons
        let button_grid_offset_y = self.text_height * 3;

        let left = (button_column(button) * button_width) as i32;
        let top = (button_row(button) * button_height + button_grid_offset_y) as i32;
        let width = button_width;
        let height = button_height;

        let rect = Rect::new(left, top, width, height);

        let depressed = game.open_menu == Some(button);
        if depressed {
            // Draw the button as depressed.
            canvas.set_draw_color(Color::from(COLOR_DEPRESSED_BUTTON_BACKGROUND));
            canvas.fill_rect(rect)?;
        }

        canvas.copy(
            &self.button_textures[button_texture_index(button)],
            None,
            Some(rect),
        )?;

        if depressed {
            canvas.set_draw_color(Color::from(COLOR_DEPRESSED_BUTTON_OUTLINE));
            canvas.draw_rect(rect)?;
        }

        Ok(())
    }
}

fn button_texture_index(button: SidebarMenu) -> usize {
    match button {
        SidebarMenu::Close => 0,
        SidebarMenu::Save => 1,
        SidebarMenu::Music => 2,
        SidebarMenu::Graphics => 3,
        SidebarMenu::Help => 4,

        SidebarMenu::Rotation => 5,
        SidebarMenu::Metrics => 6,
        SidebarMenu::Finances => 7,
        SidebarMenu::News => 8,
        SidebarMenu::Info => 9,

        SidebarMenu::Navigation => 10,
        SidebarMenu::Building => 11,
        SidebarMenu::Rail => 12,
        SidebarMenu::Demolish => 13,
        SidebarMenu::Point => 14,
    }
}

fn button_row(button: SidebarMenu) -> u32 {
    match button {
        SidebarMenu::Close
        | SidebarMenu::Save
        | SidebarMenu::Music
        | SidebarMenu::Graphics
        | SidebarMenu::Help => 0,

        SidebarMenu::Rotation
        | SidebarMenu::Metrics
        | SidebarMenu::Finances
        | SidebarMenu::News
        | SidebarMenu::Info => 1,

        SidebarMenu::Navigation
        | SidebarMenu::Building
        | SidebarMenu::Rail
        | SidebarMenu::Demolish
        | SidebarMenu::Point => 2,
    }
}

fn button_column(button: SidebarMenu) -> u32 {
    match button {
        SidebarMenu::Close => 0,
        SidebarMenu::Save => 1,
        SidebarMenu::Music => 2,
        SidebarMenu::Graphics => 3,
        SidebarMenu::Help => 4,

        SidebarMenu::Rotation => 0,
        SidebarMenu::Metrics => 1,
        SidebarMenu::Finances => 2,
        SidebarMenu::News => 3,
        SidebarMenu::Info => 4,

        SidebarMenu::Navigation => 0,
        SidebarMenu::Building => 1,
        SidebarMenu::Rail => 2,
        SidebarMenu::Demolish => 3,
        SidebarMenu::Point => 4,
    }
}
