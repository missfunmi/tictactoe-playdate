use crate::constants::{CROSS_Z_INDEX, NOUGHT_Z_INDEX, WINNING_COMBINATIONS};
use crate::enums::LevelWinner::{Computer, Neither, User};
use crate::enums::{LevelId, LevelWinner, Quadrant, SpriteType};
use crate::game_state::GameState;
use crate::graphics::GraphicsManager;
use alloc::boxed::Box;
use anyhow::Error;
use crankstart::graphics::Bitmap;
use crankstart::sprite::SpriteManager;
use crankstart::system::System;
use crankstart_sys::LCDBitmapFlip;

/*
 All game logic assumes the following positions for TicTacToe squares:

     0 | 1 | 2
    --- --- ---
     3 | 4 | 5
    --- --- ---
     6 | 7 | 8
*/
pub trait Level {
    fn get_level_id(&self) -> LevelId;

    fn next_level(&self) -> Box<dyn Level>;

    fn get_instructions(&self, graphics_manager: &GraphicsManager) -> Bitmap;

    fn human_play(
        &mut self,
        player_position: (f32, f32),
        player_selection: u8,
        game_state: &mut GameState,
        graphics_manager: &GraphicsManager,
    ) -> Result<bool, Error> {
        if game_state.computer_entries.contains(&player_selection) {
            return Ok(false);
        }
        let has_played = self.default_human_play(
            player_position,
            player_selection,
            game_state,
            graphics_manager,
        )?;
        Ok(has_played)
    }

    fn computer_play(
        &mut self,
        game_state: &mut GameState,
        graphics_manager: &GraphicsManager,
    ) -> Result<(), Error> {
        self.default_computer_play(game_state, graphics_manager)
    }

    fn is_level_over(&self, game_state: &mut GameState) -> bool {
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

    fn get_level_winner(&self, game_state: &GameState) -> LevelWinner {
        match game_state.level_winner {
            User => return User,
            Computer => return Computer,
            _ => {}
        }

        let computer_plays = &game_state.computer_entries;
        for combo in WINNING_COMBINATIONS {
            if combo.iter().all(|c| computer_plays.contains(c)) {
                return Computer;
            }
        }

        let human_plays = &game_state.player_entries;
        for combo in WINNING_COMBINATIONS {
            if combo.iter().all(|c| human_plays.contains(c)) {
                return User;
            }
        }

        Neither
    }

    fn default_human_play(
        &mut self,
        player_position: (f32, f32),
        player_selection: u8,
        game_state: &mut GameState,
        graphics_manager: &GraphicsManager,
    ) -> Result<bool, Error> {
        if !game_state.remaining_plays.contains(&player_selection) {
            return Ok(false);
        }

        let sprite_manager = SpriteManager::get_mut();

        let mut cross = sprite_manager.new_sprite()?;
        cross.set_image(
            graphics_manager.cross_image.clone(),
            LCDBitmapFlip::kBitmapUnflipped,
        )?;

        cross.set_z_index(CROSS_Z_INDEX)?;
        cross.set_opaque(false)?;
        cross.move_to(player_position.0, player_position.1)?;

        sprite_manager.add_sprite(&cross)?;
        cross.set_tag(SpriteType::Cross as u8)?;

        // VERY IMPORTANT LINE! Without this, the sprite won't be drawn on screen
        let _ = &game_state.crosses.push(cross);

        let _ = &game_state.player_entries.insert(player_selection);
        let _ = &game_state.remaining_plays.remove(&player_selection);
        game_state.last_play_time = System::get().get_elapsed_time()?;
        Ok(true)
    }

    fn default_computer_play(
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
}
