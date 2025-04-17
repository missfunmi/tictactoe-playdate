use crate::constants::PLAYER_Z_INDEX;
use crate::enums::TurnPhase::PlayersTurn;
use crate::enums::{Quadrant, SpriteType};
use crate::game_state::GameState;
use crate::graphics::GraphicsManager;
use anyhow::Error;
use crankstart::sprite::{Sprite, SpriteManager};
use crankstart::system::System;
use crankstart::Playdate;
use crankstart_sys::{LCDBitmapFlip, PDButtons};

pub struct PlayerHandler {
    pub player: Sprite
}

impl PlayerHandler {
    pub fn new(graphics_manager: &GraphicsManager) -> Result<Self, Error> {
        let sprite_manager = SpriteManager::get_mut();

        let mut player = sprite_manager.new_sprite()?;

        player.set_image(graphics_manager.player_image.clone(), LCDBitmapFlip::kBitmapUnflipped)?;
        player.move_to(Quadrant::Center.to_location().0, Quadrant::Center.to_location().1)?;
        player.set_z_index(PLAYER_Z_INDEX)?;
        player.set_opaque(true)?;
        player.set_tag(SpriteType::Player as u8)?;
        sprite_manager.add_sprite(&player)?;

        Ok(Self {
            player
        })
    }

    pub fn update(&mut self, sprite: &mut Sprite, game_state: &GameState, _playdate: &Playdate) -> Result<(), Error> {
        if game_state.level_over || game_state.turn_phase != PlayersTurn {
            return Ok(());
        }

        let (_, pushed, _) = System::get().get_button_state()?;
        let current_position = sprite.get_position()?;
        let current_quadrant = Quadrant::from_location(current_position.0, current_position.1);

        let new_quadrant;

        if (pushed & PDButtons::kButtonUp) == PDButtons::kButtonUp {
            new_quadrant = current_quadrant.move_up();
        } else if (pushed & PDButtons::kButtonDown) == PDButtons::kButtonDown {
            new_quadrant = current_quadrant.move_down();
        } else if (pushed & PDButtons::kButtonLeft) == PDButtons::kButtonLeft {
            new_quadrant = current_quadrant.move_left();
        } else if (pushed & PDButtons::kButtonRight) == PDButtons::kButtonRight {
            new_quadrant = current_quadrant.move_right();
        } else {
            new_quadrant = current_quadrant;
        }

        let new_location = new_quadrant.to_location();
        sprite.move_to(new_location.0, new_location.1)?;
        Ok(())
    }
}
