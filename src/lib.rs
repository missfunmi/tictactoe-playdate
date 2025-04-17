#![no_std]

extern crate alloc;
mod background;
mod constants;
mod enums;
mod game_state;
mod graphics;
mod level_1;
mod level_2;
mod level_3;
mod level_4;
mod level_5;
mod level_6;
mod level_7;
mod levels;
mod player;

use crate::constants::{INTERACTION_DELAY, LEVEL_OVERLAY_INDEX};
use crate::enums::{LevelWinner, TurnPhase};
use crate::level_1::LevelOne;
use crate::levels::Level;
use alloc::boxed::Box;
use anyhow::Error;
use background::BackgroundHandler;
use crankstart::display::Display;
use crankstart::sprite::{Sprite, SpriteManager};
use crankstart::system::System;
use crankstart::{crankstart_game, Game, Playdate};
use crankstart_sys::{LCDBitmapFlip, PDButtons, PDRect};
use enums::{Quadrant, SpriteType};
use game_state::GameState;
use graphics::GraphicsManager;
use player::PlayerHandler;

struct TicTacToeGame {
    game_state: GameState,
    graphics_manager: GraphicsManager,
    background_handler: BackgroundHandler,
    player_handler: PlayerHandler,
    level: Box<dyn Level>,
}

impl TicTacToeGame {
    pub fn new(_playdate: &Playdate) -> Result<Box<Self>, Error> {
        let mut game = Self::initialize_game()?;
        game.game_state.reset_game()?;
        Ok(Box::new(game))
    }

    fn initialize_game() -> Result<Self, Error> {
        Display::get().set_refresh_rate(20.0)?;

        let graphics_manager = GraphicsManager::new()?;
        let player_handler = PlayerHandler::new(&graphics_manager)?;
        let background_handler = BackgroundHandler::new(&graphics_manager)?;
        let game_state = GameState::new();
        let level = Box::new(LevelOne::new());

        let game = Self {
            game_state,
            background_handler,
            player_handler,
            graphics_manager,
            level,
        };

        Ok(game)
    }

    fn handle_level_start(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        let (_, _, released) = System::get().get_button_state()?;

        if (released & PDButtons::kButtonA) == PDButtons::kButtonA
            || (released & PDButtons::kButtonB) == PDButtons::kButtonB {
            self.game_state.overlays.clear();
            self.game_state.turn_phase = TurnPhase::PlayersTurn;
            return Ok(());
        }

        let level_start_image = self.level.get_instructions(&self.graphics_manager);

        let sprite_manager = SpriteManager::get_mut();
        let mut level_start = sprite_manager.new_sprite()?;
        level_start.set_image(level_start_image, LCDBitmapFlip::kBitmapUnflipped)?;
        level_start.set_z_index(LEVEL_OVERLAY_INDEX)?;
        level_start.move_to(
            Quadrant::Center.to_location().0,
            Quadrant::Center.to_location().1,
        )?;
        level_start.set_tag(SpriteType::LevelStart as u8)?;
        sprite_manager.add_sprite(&level_start)?;

        self.game_state.overlays.push(level_start);

        Ok(())
    }

    fn handle_user_play(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        let (_, _, released) = System::get().get_button_state()?;

        if (released & PDButtons::kButtonA) == PDButtons::kButtonA {
            let player_position = self.player_handler.player.get_position()?;
            let selected_quadrant = Quadrant::from_location(player_position.0, player_position.1);
            let selection = selected_quadrant as u8;

            let is_user_play_completed =
                self.level.human_play(player_position, selection, &mut self.game_state, &self.graphics_manager)?;

            match is_user_play_completed {
                true => {
                    let is_level_over = self.level.is_level_over(&mut self.game_state);
                    let timestamp = System::get().get_elapsed_time()?;
                    match is_level_over {
                        true => self.game_state.turn_phase = TurnPhase::LevelOver { timestamp },
                        false => self.game_state.turn_phase = TurnPhase::ComputersTurn { timestamp },
                    }
                }
                false => {
                    self.game_state.turn_phase = TurnPhase::PlayersTurn
                }
            }
        }
        Ok(())
    }

    fn handle_computer_play(&mut self) -> Result<(), Error> {
        self.level.computer_play(&mut self.game_state, &self.graphics_manager)?;

        let is_level_over = self.level.is_level_over(&mut self.game_state);
        match is_level_over {
            true => {
                let timestamp = System::get().get_elapsed_time()?;
                self.game_state.turn_phase = TurnPhase::LevelOver { timestamp }
            },
            false => self.game_state.turn_phase = TurnPhase::PlayersTurn,
        }

        Ok(())
    }

    fn handle_level_over(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        if !self.game_state.level_over {
            return Ok(());
        }

        let (_, _, released) = System::get().get_button_state()?;
        if (released & PDButtons::kButtonA) == PDButtons::kButtonA {
            let is_game_over = self.has_game_ended();
            let timestamp = System::get().get_elapsed_time()?;
            if is_game_over {
                self.game_state.turn_phase = TurnPhase::GameOver { timestamp };
                return Ok(());
            }

            let next_level = self.level.next_level();
            self.level = next_level;
            self.game_state.reset_game()?;
            return Ok(());
        } else if (released & PDButtons::kButtonB) == PDButtons::kButtonB {
            self.game_state.reset_game()?;
            self.game_state.turn_phase = TurnPhase::PlayersTurn;
            return Ok(());
        }

        self.game_state.level_winner = self.level.get_level_winner(&self.game_state);

        let level_over_image = match self.game_state.level_winner {
            LevelWinner::User => self.graphics_manager.player_wins_image.clone(),
            LevelWinner::Computer => self.graphics_manager.player_loses_image.clone(),
            LevelWinner::Neither => self.graphics_manager.game_tied_image.clone(),
        };

        let sprite_manager = SpriteManager::get_mut();
        let mut level_over = sprite_manager.new_sprite()?;
        level_over.set_image(level_over_image, LCDBitmapFlip::kBitmapUnflipped)?;
        level_over.set_z_index(LEVEL_OVERLAY_INDEX)?;
        level_over.move_to(
            Quadrant::Center.to_location().0,
            Quadrant::Center.to_location().1,
        )?;
        level_over.set_tag(SpriteType::LevelOver as u8)?;
        sprite_manager.add_sprite(&level_over)?;

        self.game_state.overlays.push(level_over);

        Ok(())
    }

    fn handle_game_over(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        if !self.game_state.level_over && !self.has_game_ended() {
            return Ok(());
        }

        let (_, _, released) = System::get().get_button_state()?;
        if (released & PDButtons::kButtonA) == PDButtons::kButtonA
            || (released & PDButtons::kButtonB) == PDButtons::kButtonB {
            self.level = Box::new(LevelOne::new());
            self.game_state.reset_game()?;
            return Ok(());
        }

        let game_over_image = self.graphics_manager.game_over_image.clone();

        let sprite_manager = SpriteManager::get_mut();
        let mut game_over = sprite_manager.new_sprite()?;
        game_over.set_image(game_over_image, LCDBitmapFlip::kBitmapUnflipped)?;
        game_over.set_z_index(LEVEL_OVERLAY_INDEX)?;
        game_over.move_to(
            Quadrant::Center.to_location().0,
            Quadrant::Center.to_location().1,
        )?;
        game_over.set_tag(SpriteType::GameOver as u8)?;
        sprite_manager.add_sprite(&game_over)?;

        self.game_state.overlays.push(game_over);

        Ok(())
    }

    fn has_game_ended(&mut self) -> bool {
        let is_game_over =
            matches!(self.game_state.turn_phase, TurnPhase::LevelOver { .. })
                && self.level.get_level_id() == self.level.next_level().get_level_id();
        is_game_over
    }
}

impl Game for TicTacToeGame {
    fn update_sprite(&mut self, sprite: &mut Sprite, playdate: &mut Playdate) -> Result<(), Error> {
        let tag = sprite.get_tag()?.into();
        match tag {
            SpriteType::Player => self.player_handler.update(sprite, &self.game_state, playdate)?,
            _ => {}
        }

        sprite.mark_dirty()?;
        Ok(())
    }

    fn draw_sprite(
        &self,
        sprite: &Sprite,
        _bounds: &PDRect,
        _draw_rect: &PDRect,
        _playdate: &Playdate,
    ) -> Result<(), Error> {
        let tag = sprite.get_tag()?.into();
        match tag {
            SpriteType::Background => self.background_handler.draw_background()?,
            _ => {}
        }
        Ok(())
    }

    fn update(&mut self, playdate: &mut Playdate) -> Result<(), Error> {
        match self.game_state.turn_phase {
            TurnPhase::LevelStart { timestamp } => {
                let now = System::get().get_elapsed_time()?;
                if now - timestamp >= INTERACTION_DELAY {
                    self.handle_level_start(playdate)?;
                }
            }
            TurnPhase::PlayersTurn => {
                self.handle_user_play(playdate)?;
            }
            TurnPhase::ComputersTurn { timestamp } => {
                let now = System::get().get_elapsed_time()?;
                if now - timestamp >= INTERACTION_DELAY {
                    self.handle_computer_play()?;
                }
            }
            TurnPhase::LevelOver { timestamp } => {
                let now = System::get().get_elapsed_time()?;
                if now - timestamp >= INTERACTION_DELAY {
                    self.handle_level_over(playdate)?;
                }
            }
            TurnPhase::GameOver { timestamp } => {
                let now = System::get().get_elapsed_time()?;
                if now - timestamp >= INTERACTION_DELAY {
                    self.handle_game_over(playdate)?;
                }
            }
        }

        Ok(())
    }

    fn draw_fps(&self) -> bool {
        false
    }

    fn draw_and_update_sprites(&self) -> bool {
        true
    }
}

crankstart_game!(TicTacToeGame);
