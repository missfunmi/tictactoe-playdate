use crate::enums::{LevelId, LevelWinner};
use crate::game_state::GameState;
use crate::graphics::GraphicsManager;
use crate::level_6::LevelSix;
use crate::levels::Level;
use alloc::boxed::Box;
use anyhow::Error;
use crankstart::graphics::Bitmap;

// 5: reverse disappearing tictactoe | rules 2 and 3 combined
pub struct LevelFive {}

impl LevelFive {
    pub fn new() -> Self {
        Self {}
    }
}

impl Level for LevelFive {
    fn get_level_id(&self) -> LevelId {
        LevelId::Five
    }

    fn next_level(&self) -> Box<dyn Level> {
        Box::new(LevelSix::new())
    }

    fn get_instructions(&self, graphics_manager: &GraphicsManager) -> Bitmap {
        graphics_manager.level_5_image.clone()
    }

    fn human_play(
        &mut self,
        player_position: (f32, f32),
        player_selection: u8,
        game_state: &mut GameState,
        graphics_manager: &GraphicsManager,
    ) -> Result<bool, Error> {
        if game_state.player_entries.contains(&player_selection) {
            return Ok(false)
        }

        game_state.crosses.pop();
        self.default_human_play(player_position, player_selection, game_state, graphics_manager)?;
        Ok(true)
    }

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