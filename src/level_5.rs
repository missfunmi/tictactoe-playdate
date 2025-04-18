use crate::enums::{LevelId, LevelWinner, Quadrant, SpriteType};
use crate::game_state::GameState;
use crate::graphics::GraphicsManager;
use crate::level_6::LevelSix;
use crate::levels::Level;
use alloc::boxed::Box;
use anyhow::Error;
use crankstart::graphics::Bitmap;
use crankstart::sprite::SpriteManager;
use crankstart::system::System;
use crankstart_sys::LCDBitmapFlip;
use crate::constants::NOUGHT_Z_INDEX;

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