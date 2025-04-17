use crate::constants::{NOUGHT_Z_INDEX, WINNING_COMBINATIONS};
use crate::enums::LevelWinner::Computer;
use crate::enums::{LevelId, Quadrant, SpriteType};
use crate::game_state::GameState;
use crate::graphics::GraphicsManager;
use crate::level_5::LevelFive;
use crate::levels::Level;
use alloc::boxed::Box;
use anyhow::Error;
use crankstart::graphics::Bitmap;
use crankstart::sprite::SpriteManager;
use crankstart::system::System;
use crankstart_sys::LCDBitmapFlip;

pub struct LevelFour {}

// 4: unforgiving disappearing tictactoe | same as level 3, except if you try to play
// in a cell already filled you lose
impl LevelFour {
    pub fn new() -> Self {
        Self {}
    }
}

impl Level for LevelFour {
    fn get_level_id(&self) -> LevelId {
        LevelId::Four
    }

    fn next_level(&self) -> Box<dyn Level> {
        Box::new(LevelFive::new())
    }

    fn get_instructions(&self, graphics_manager: &GraphicsManager) -> Bitmap {
        graphics_manager.level_4_image.clone()
    }

    fn human_play(
        &mut self,
        player_position: (f32, f32),
        player_selection: u8,
        game_state: &mut GameState,
        graphics_manager: &GraphicsManager,
    ) -> Result<bool, Error> {

        if !game_state.remaining_plays.contains(&player_selection) {
            game_state.level_over = true;
            game_state.level_winner = Computer;
            return Ok(true);
        }

        game_state.crosses.pop();
        self.default_human_play(player_position, player_selection, game_state, graphics_manager)?;
        Ok(true)
    }

    fn computer_play(
        &mut self,
        game_state: &mut GameState,
        graphics_manager: &GraphicsManager,
    ) -> Result<(), Error> {
        if game_state.level_over {
            return Ok(());
        }

        let potential_computer_play = game_state.remaining_plays.iter().next().cloned();

        match potential_computer_play {
            Some(num) => {
                game_state.noughts.pop();
                let computer_play = num;
                let computer_play_quadrant = Quadrant::from(computer_play);
                let computer_move = computer_play_quadrant.to_location();

                let sprite_manager = SpriteManager::get_mut();
                let mut nought = sprite_manager.new_sprite()?;
                nought.set_image(
                    graphics_manager.nought_image.clone(),
                    LCDBitmapFlip::kBitmapUnflipped,
                )?;

                nought.set_z_index(NOUGHT_Z_INDEX)?;
                nought.set_opaque(false)?;
                nought.move_to(computer_move.0, computer_move.1)?;

                sprite_manager.add_sprite(&nought)?;
                nought.set_tag(SpriteType::Nought as u8)?;

                game_state.noughts.push(nought);

                game_state.computer_entries.insert(computer_play);
                game_state.remaining_plays.remove(&computer_play);
                game_state.last_play_time = System::get().get_elapsed_time()?;
            }
            None => {}
        }

        Ok(())
    }

    fn is_level_over(&self, game_state: &mut GameState) -> bool {
        if game_state.level_over {
            return true;
        }

        let remaining_plays = &game_state.remaining_plays;

        if remaining_plays.is_empty() {
            game_state.level_over = true;
            return true;
        }

        let computer_plays = &game_state.computer_entries;
        for combo in WINNING_COMBINATIONS {
            if combo.iter().all(|c| computer_plays.contains(c)) {
                game_state.level_over = true;
                return true;
            }
        }

        let human_plays = &game_state.player_entries;
        for combo in WINNING_COMBINATIONS {
            if combo.iter().all(|c| human_plays.contains(c)) {
                game_state.level_over = true;
                return true;
            }
        }

        game_state.level_over = false;
        false
    }

}