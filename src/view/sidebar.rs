use crate::action::GameAction;
use crate::controller::{PlayerAction, WindowPanel};
use crate::state::{Direction, GameState, PlayerMode, SidebarMenu};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::Window;

const COLOR_SIDEBAR: (u8, u8, u8) = (132, 132, 123);
const COLOR_DEPRESSED_BUTTON_OUTLINE: (u8, u8, u8) = (255, 0, 0);
const COLOR_DEPRESSED_BUTTON_BACKGROUND: (u8, u8, u8, u8) = (255, 255, 255, 100);

const BUTTON_GRID_OFFSET_Y: u32 = 60;
const BUTTON_WIDTH: u32 = 32;
const BUTTON_HEIGHT: u32 = 32;

pub struct Sidebar {
    scale_x: u32,
    scale_y: u32,
    width: u32,
    height: u32,
    button_textures: [Texture; 15],
}

impl Sidebar {
    pub fn new<T>(
        texture_creator: TextureCreator<T>,
        width: u32,
        height: u32,
        scale_x: u32,
        scale_y: u32,
    ) -> Self {
        Self {
            scale_x,
            scale_y,
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
                texture_creator.load_texture("art/dozer_128.png").unwrap(),
                texture_creator.load_texture("art/point_128.png").unwrap(),
            ],
        }
    }

    pub fn map_player_action(
        &self,
        game: &GameState,
        player_action: PlayerAction,
    ) -> Option<GameAction> {
        match player_action {
            PlayerAction::CursorMove {
                panel: WindowPanel::Sidebar,
                x,
                y,
            } => Some(GameAction::SidebarHover {
                button: self.button_under_cursor(x, y),
            }),
            PlayerAction::WindowLeftClick {
                panel: WindowPanel::Sidebar,
                x,
                y,
            } => match game.highlighted_button {
                Some(menu) => Some(GameAction::OpenMenu { menu }),
                None => None,
            },
            _ => None,
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
        match (
            x / (BUTTON_WIDTH * self.scale_x) as i32,
            (y - (BUTTON_GRID_OFFSET_Y * self.scale_y) as i32)
                / (BUTTON_HEIGHT * self.scale_y) as i32,
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
        let left = (button_column(button) * BUTTON_WIDTH * self.scale_x) as i32;
        let top = (button_row(button) * BUTTON_HEIGHT * self.scale_y
            + BUTTON_GRID_OFFSET_Y * self.scale_y) as i32;
        let width = BUTTON_WIDTH * self.scale_x;
        let height = BUTTON_HEIGHT * self.scale_y;

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
