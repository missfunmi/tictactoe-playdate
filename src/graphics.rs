use anyhow::Error;
use crankstart::graphics::{Bitmap, Graphics};

pub struct GraphicsManager {
    pub background_image: Bitmap,
    pub cross_image: Bitmap,
    pub game_over_image: Bitmap,
    pub game_tied_image: Bitmap,
    pub level_1_image: Bitmap,
    pub level_2_image: Bitmap,
    pub level_3_image: Bitmap,
    pub level_4_image: Bitmap,
    pub level_5_image: Bitmap,
    pub level_6_image: Bitmap,
    pub level_7_image: Bitmap,
    pub nought_image: Bitmap,
    pub player_image: Bitmap,
    pub player_loses_image: Bitmap,
    pub player_wins_image: Bitmap,
}

impl GraphicsManager {
    pub fn new() -> Result<Self, Error> {
        let graphics = Graphics::get();

        let background_image = graphics.load_bitmap("assets/_background")?;
        let cross_image = graphics.load_bitmap("assets/_cross")?;
        let game_over_image = graphics.load_bitmap("assets/_game_over")?;
        let game_tied_image = graphics.load_bitmap("assets/_game_tied")?;
        let level_1_image = graphics.load_bitmap("assets/_level_1")?;
        let level_2_image = graphics.load_bitmap("assets/_level_2")?;
        let level_3_image = graphics.load_bitmap("assets/_level_3")?;
        let level_4_image = graphics.load_bitmap("assets/_level_4")?;
        let level_5_image = graphics.load_bitmap("assets/_level_5")?;
        let level_6_image = graphics.load_bitmap("assets/_level_6")?;
        let level_7_image = graphics.load_bitmap("assets/_level_7")?;
        let nought_image = graphics.load_bitmap("assets/_nought")?;
        let player_image = graphics.load_bitmap("assets/_hand")?;
        let player_loses_image = graphics.load_bitmap("assets/_player_loses")?;
        let player_wins_image = graphics.load_bitmap("assets/_player_wins")?;

        Ok(Self {
            background_image,
            cross_image,
            game_over_image,
            game_tied_image,
            level_1_image,
            level_2_image,
            level_3_image,
            level_4_image,
            level_5_image,
            level_6_image,
            level_7_image,
            nought_image,
            player_image,
            player_loses_image,
            player_wins_image,
        })
    }
}
