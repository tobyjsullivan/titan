use crate::action::GameAction;
use crate::controller::{PlayerAction, WindowPanel};
use crate::state::{Direction, GameState, Object, PlayerMode, SidebarButton};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::Window;

const COLOR_SIDEBAR: (u8, u8, u8) = (132, 132, 123);
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
            } => {
                match game.highlighted_button {
                    Some(SidebarButton::Navigation) => Some(GameAction::SetPlayerMode {
                        mode: PlayerMode::Focus,
                    }),
                    Some(SidebarButton::Building) => Some(GameAction::SetPlayerMode {
                        mode: PlayerMode::PlaceObject {
                            obj: Object::Forest,
                            orientation: Direction::North,
                        },
                    }),
                    Some(SidebarButton::Demolish) => Some(GameAction::SetPlayerMode {
                        mode: PlayerMode::RaiseLower { radius: 0 },
                    }),
                    // TODO (toby)
                    Some(_) => None,
                    None => None,
                }
            }
            _ => None,
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, game: &GameState) -> Result<(), String> {
        canvas.set_draw_color(Color::from(COLOR_SIDEBAR));
        canvas.fill_rect(Rect::new(0, 0, self.width, self.height))?;

        // Draw buttons
        self.draw_button(canvas, &SidebarButton::Close)?;
        self.draw_button(canvas, &SidebarButton::Save)?;
        self.draw_button(canvas, &SidebarButton::Music)?;
        self.draw_button(canvas, &SidebarButton::Graphics)?;
        self.draw_button(canvas, &SidebarButton::Help)?;

        self.draw_button(canvas, &SidebarButton::Rotation)?;
        self.draw_button(canvas, &SidebarButton::Metrics)?;
        self.draw_button(canvas, &SidebarButton::Finances)?;
        self.draw_button(canvas, &SidebarButton::News)?;
        self.draw_button(canvas, &SidebarButton::Info)?;

        self.draw_button(canvas, &SidebarButton::Navigation)?;
        self.draw_button(canvas, &SidebarButton::Building)?;
        self.draw_button(canvas, &SidebarButton::Rail)?;
        self.draw_button(canvas, &SidebarButton::Demolish)?;
        self.draw_button(canvas, &SidebarButton::Point)?;

        Ok(())
    }

    fn button_under_cursor(&self, x: i32, y: i32) -> Option<SidebarButton> {
        match (
            x / (BUTTON_WIDTH * self.scale_x) as i32,
            (y - (BUTTON_GRID_OFFSET_Y * self.scale_y) as i32)
                / (BUTTON_HEIGHT * self.scale_y) as i32,
        ) {
            (0, 0) => Some(SidebarButton::Close),
            (1, 0) => Some(SidebarButton::Save),
            (2, 0) => Some(SidebarButton::Music),
            (3, 0) => Some(SidebarButton::Graphics),
            (4, 0) => Some(SidebarButton::Help),

            (0, 1) => Some(SidebarButton::Rotation),
            (1, 1) => Some(SidebarButton::Metrics),
            (2, 1) => Some(SidebarButton::Finances),
            (3, 1) => Some(SidebarButton::News),
            (4, 1) => Some(SidebarButton::Info),

            (0, 2) => Some(SidebarButton::Navigation),
            (1, 2) => Some(SidebarButton::Building),
            (2, 2) => Some(SidebarButton::Rail),
            (3, 2) => Some(SidebarButton::Demolish),
            (4, 2) => Some(SidebarButton::Point),

            (_, _) => None,
        }
    }

    fn draw_button(
        &self,
        canvas: &mut Canvas<Window>,
        button: &SidebarButton,
    ) -> Result<(), String> {
        canvas.copy(
            &self.button_textures[button_texture_index(button)],
            None,
            Some(Rect::new(
                (button_column(button) * BUTTON_WIDTH * self.scale_x) as i32,
                (button_row(button) * BUTTON_HEIGHT * self.scale_y
                    + BUTTON_GRID_OFFSET_Y * self.scale_y) as i32,
                BUTTON_WIDTH * self.scale_x,
                BUTTON_HEIGHT * self.scale_y,
            )),
        )
    }
}

fn button_texture_index(button: &SidebarButton) -> usize {
    match button {
        SidebarButton::Close => 0,
        SidebarButton::Save => 1,
        SidebarButton::Music => 2,
        SidebarButton::Graphics => 3,
        SidebarButton::Help => 4,

        SidebarButton::Rotation => 5,
        SidebarButton::Metrics => 6,
        SidebarButton::Finances => 7,
        SidebarButton::News => 8,
        SidebarButton::Info => 9,

        SidebarButton::Navigation => 10,
        SidebarButton::Building => 11,
        SidebarButton::Rail => 12,
        SidebarButton::Demolish => 13,
        SidebarButton::Point => 14,
    }
}

fn button_row(button: &SidebarButton) -> u32 {
    match button {
        SidebarButton::Close
        | SidebarButton::Save
        | SidebarButton::Music
        | SidebarButton::Graphics
        | SidebarButton::Help => 0,

        SidebarButton::Rotation
        | SidebarButton::Metrics
        | SidebarButton::Finances
        | SidebarButton::News
        | SidebarButton::Info => 1,

        SidebarButton::Navigation
        | SidebarButton::Building
        | SidebarButton::Rail
        | SidebarButton::Demolish
        | SidebarButton::Point => 2,
    }
}

fn button_column(button: &SidebarButton) -> u32 {
    match button {
        SidebarButton::Close => 0,
        SidebarButton::Save => 1,
        SidebarButton::Music => 2,
        SidebarButton::Graphics => 3,
        SidebarButton::Help => 4,

        SidebarButton::Rotation => 0,
        SidebarButton::Metrics => 1,
        SidebarButton::Finances => 2,
        SidebarButton::News => 3,
        SidebarButton::Info => 4,

        SidebarButton::Navigation => 0,
        SidebarButton::Building => 1,
        SidebarButton::Rail => 2,
        SidebarButton::Demolish => 3,
        SidebarButton::Point => 4,
    }
}
