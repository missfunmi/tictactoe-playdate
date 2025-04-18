use crate::constants::NOUGHT_Z_INDEX;
use crate::enums::{LevelId, Quadrant, SpriteType};
use crate::game_state::GameState;
use crate::graphics::GraphicsManager;
use crate::level_7::LevelSeven;
use crate::levels::Level;
use alloc::boxed::Box;
use anyhow::Error;
use crankstart::graphics::Bitmap;
use crankstart::log_to_console;
use crankstart::sprite::SpriteManager;
use crankstart::system::System;
use crankstart_sys::LCDBitmapFlip;

// 6: rearranging tictactoe | every turn, the x's and o's are shuffled
pub struct LevelSix {}

impl LevelSix {
    pub fn new() -> Self {
        Self {}
    }

    fn randomize_user_last_play(game_state: &mut GameState) -> Result<(), Error> {
        let player_entries = &game_state.player_entries;
        let player_entries_length = player_entries.len();

        if player_entries_length > 0 {
            let last_cross = game_state.crosses.last_mut().unwrap();

            let current_location = last_cross.get_position()?;
            let current_location_quadrant =
                Quadrant::from_location(current_location.0, current_location.1);
            let current_location_play = current_location_quadrant as u8;

            let random_new_play = game_state.remaining_plays.iter().next().unwrap().clone();
            let random_new_play_quadrant = Quadrant::from(random_new_play);
            let random_new_play_to_location = random_new_play_quadrant.to_location();

            log_to_console!(
                        "cross currently at: {:?}, will move to: {:?}. pre-move state of \
                        remaining_plays: {:?}, player_entries: {:?}, computer_entries: {:?}",
                        current_location_play,
                        random_new_play,
                        game_state.remaining_plays,
                        game_state.player_entries,
                        game_state.computer_entries,
                    );

            last_cross
                .move_to(random_new_play_to_location.0, random_new_play_to_location.1)?;

            game_state.player_entries.remove(&current_location_play);
            game_state.player_entries.insert(random_new_play);

            game_state.remaining_plays.insert(current_location_play);
            game_state.remaining_plays.remove(&random_new_play);

            log_to_console!(
                        "post-move state of \
                        remaining_plays: {:?}, player_entries: {:?}, computer_entries: {:?}",
                        game_state.remaining_plays,
                        game_state.player_entries,
                        game_state.computer_entries,
                    );
        }
        Ok(())
    }
}

impl Level for LevelSix {
    fn get_level_id(&self) -> LevelId {
        LevelId::Six
    }

    fn next_level(&self) -> Box<dyn Level> {
        Box::new(LevelSeven::new())
    }

    fn get_instructions(&self, graphics_manager: &GraphicsManager) -> Bitmap {
        graphics_manager.level_6_image.clone()
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

                // For Level 6 only -- move last player cross to a random new location
                Self::randomize_user_last_play(game_state)?;
            }
            None => {}
        }

        Ok(())
    }
}
