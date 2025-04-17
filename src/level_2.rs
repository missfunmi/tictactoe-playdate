use crate::enums::{LevelId, LevelWinner};
use crate::game_state::GameState;
use crate::graphics::GraphicsManager;
use crate::level_3::LevelThree;
use crate::levels::Level;
use alloc::boxed::Box;
use crankstart::graphics::Bitmap;

pub struct LevelTwo {}

// 2: reverse tictactoe | reverse of basic game play, where player loses if they get any 3 in a row
impl LevelTwo {
    pub fn new() -> Self {
        Self {}
    }
}

impl Level for LevelTwo {
    fn get_level_id(&self) -> LevelId {
        LevelId::Two
    }

    fn next_level(&self) -> Box<dyn Level> {
        Box::new(LevelThree::new())
    }

    fn get_instructions(&self, graphics_manager: &GraphicsManager) -> Bitmap {
        graphics_manager.level_2_image.clone()
    }

    // Rules are reversed in Level 2
    fn get_level_winner(&self, game_state: &GameState) -> LevelWinner {
        let computer_plays = &game_state.computer_entries;
        for combo in crate::constants::WINNING_COMBINATIONS {
            if combo.iter().all(|c| computer_plays.contains(c)) {
                return LevelWinner::User;
            }
        }

        let human_plays = &game_state.player_entries;
        for combo in crate::constants::WINNING_COMBINATIONS {
            if combo.iter().all(|c| human_plays.contains(c)) {
                return LevelWinner::Computer;
            }
        }

        LevelWinner::Neither
    }
}
