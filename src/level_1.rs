use crate::enums::LevelId;
use crate::graphics::GraphicsManager;
use crate::level_2::LevelTwo;
use crate::levels::Level;
use alloc::boxed::Box;
use crankstart::graphics::Bitmap;

pub struct LevelOne {}

// 1: basic tictactoe | computer selects a move randomly for the 'O'
// currently, it almost never picks an optimal or sensible move
impl LevelOne {
    pub fn new() -> Self {
        Self {}
    }
}

impl Level for LevelOne {
    fn get_level_id(&self) -> LevelId {
        LevelId::One
    }

    fn next_level(&self) -> Box<dyn Level> {
        Box::new(LevelTwo::new())
    }

    fn get_instructions(&self, graphics_manager: &GraphicsManager) -> Bitmap {
        graphics_manager.level_1_image.clone()
    }
}
