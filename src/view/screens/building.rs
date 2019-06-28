use crate::state::game::GameState;
use crate::state::menu::building::{BuyBuildingScreenState, Category};
use crate::view::text::DynamicText;
use crate::view::{ScreenState, DIALOG_HEIGHT, DIALOG_WIDTH, TEXT_HEIGHT};
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator, TextureQuery};
use sdl2::video::Window;
use std::rc::Rc;

const COLOR_BACKGROUND: (u8, u8, u8) = (82, 82, 82);
const COLOR_HIGHLIGHT: (u8, u8, u8) = (255, 255, 255);
const COLOR_TEXT: (u8, u8, u8) = (255, 255, 255);
const COLOR_TEXT_HIGHLIGHTED: (u8, u8, u8) = (0, 0, 0);
const ASSET_DIMENSIONS: (u32, u32) = (2560, 1920);
const BUILDING_CATEGORIES_OFFSET: (u32, u32) = (1656, 378);
const BUILDING_SELECTION_OFFSET: (u32, u32) = (1656, 680);
const LINE_SPACING: u32 = 4;
const TEXT_AREA_WIDTH: u32 = 800;

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

    pub fn render(&self, canvas: &mut Canvas<Window>, game: &GameState) -> Result<(), String> {
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

        let (asset_width, asset_height) = ASSET_DIMENSIONS;
        let scale = dialog_width as f32 / asset_width as f32;

        let (cat_offset_x, cat_offset_y) = BUILDING_CATEGORIES_OFFSET;
        let cat_offset_x = (cat_offset_x as f32 * scale) as i32;
        let cat_offset_y = (cat_offset_y as f32 * scale) as i32;
        let line_spacing = self.screen.scale_y(LINE_SPACING as i32);
        let line_height = self.screen.scale_y(TEXT_HEIGHT as i32) + line_spacing;
        let line_width = (TEXT_AREA_WIDTH as f32 * scale) as i32;

        let selected_category = match game.buy_building_screen {
            BuyBuildingScreenState::Visible {
                selected_building,
                selected_category,
            } => Some(selected_category),
            BuyBuildingScreenState::Hidden => None,
        };
        let mut line_num = 0;
        for &cat in [
            Category::Terminals,
            Category::Production,
            Category::Retail,
            Category::CityBuildings,
        ]
        .iter()
        {
            let highlight = selected_category == Some(cat);
            let line_left = dialog_rect.left() + cat_offset_x;
            let line_top = dialog_rect.top() + cat_offset_y + line_height * line_num;

            if highlight {
                // Draw background.
                canvas.set_draw_color(Color::from(COLOR_HIGHLIGHT));
                canvas.fill_rect(Rect::new(
                    line_left,
                    line_top - line_spacing / 2,
                    line_width as u32,
                    line_height as u32,
                ))?;
            }
            self.dynamic_text.print(
                canvas,
                &Self::display_name(cat),
                Point::new(line_left, line_top),
                highlight,
            )?;
            line_num += 1;
        }

        Ok(())
    }

    fn display_name(cat: Category) -> String {
        match cat {
            Category::Terminals => String::from("Terminals"),
            Category::Production => String::from("Production"),
            Category::Retail => String::from("Retail"),
            Category::CityBuildings => String::from("Buildings (city)"),
        }
    }
}
