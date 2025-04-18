use crate::enums::{LevelId, Quadrant};
use crate::game_state::GameState;
use crate::graphics::GraphicsManager;
use crate::levels::Level;
use alloc::boxed::Box;
use anyhow::Error;
use crankstart::graphics::Bitmap;
use hashbrown::HashSet;

pub struct LevelSeven {}

// 7: murder tictactoe | you can choose to override the computer's entry instead of playing your turn
impl LevelSeven {
    pub fn new() -> Self {
        Self {}
    }
}

impl Level for LevelSeven {
    fn get_level_id(&self) -> LevelId {
        LevelId::Seven
    }

    fn next_level(&self) -> Box<dyn Level> {
        Box::new(LevelSeven::new())
    }

    fn get_instructions(&self, graphics_manager: &GraphicsManager) -> Bitmap {
        graphics_manager.level_7_image.clone()
    }

    fn human_play(
        &mut self,
        player_position: (f32, f32),
        player_selection: u8,
        game_state: &mut GameState,
        graphics_manager: &GraphicsManager,
    ) -> Result<bool, Error> {
        if game_state.computer_entries.contains(&player_selection) {
            let selected_location = Quadrant::from(player_selection).to_location();

            game_state
                .noughts
                .retain(|nought| !selected_location.eq(&nought.get_position().unwrap()));
            game_state.computer_entries.remove(&player_selection);

            // Lol, so bad
            // Temporary workaround to re-insert the "murdered" computer play back into
            // game_state.remaining_plays since I can't shuffle the hashset without more complex logic
            // TODO probably should just redo default computer_play logic in levels.rs
            let current_remaining_plays = &game_state.remaining_plays;
            let mut updated_remaining_plays = HashSet::new();
            updated_remaining_plays.insert(player_selection);
            updated_remaining_plays.extend(current_remaining_plays);
            game_state.remaining_plays = updated_remaining_plays;

            return Ok(true);
        }

        self.default_human_play(
            player_position,
            player_selection,
            game_state,
            graphics_manager,
        )?;

        Ok(true)
    }
}
