use crate::enums::SpriteType;
use crate::graphics::GraphicsManager;
use anyhow::Error;
use crankstart::graphics::{rect_make, Bitmap};
use crankstart::sprite::{Sprite, SpriteManager};
use crankstart_sys::LCDBitmapFlip;
use euclid::point2;

const BACKGROUND_Z_INDEX: i16 = 0;

pub struct BackgroundHandler {
    background_image: Bitmap,
    #[allow(unused)]
    background: Sprite
}

impl BackgroundHandler {
    pub fn new(graphics_manager: &GraphicsManager) -> Result<Self, Error> {
        let sprite_manager = SpriteManager::get_mut();

        let background_image = graphics_manager.background_image.clone();

        let mut background = sprite_manager.new_sprite()?;
        let bounds = rect_make(0.0, 0.0, 400.0, 240.0);
        background.set_bounds(&bounds)?;
        background.set_use_custom_draw()?;
        background.set_z_index(BACKGROUND_Z_INDEX)?;
        background.set_tag(SpriteType::Background as u8)?;
        sprite_manager.add_sprite(&background)?;

        Ok(Self {
            background_image,
            background
        })
    }

    pub fn draw_background(&self) -> Result<(), Error> {
        self.background_image.draw(point2(0, 0), LCDBitmapFlip::kBitmapUnflipped)?;
        Ok(())
    }
}