use crate::enums::{LevelWinner, TurnPhase};
use alloc::vec::Vec;
use anyhow::Error;
use crankstart::sprite::Sprite;
use crankstart::system::System;
use hashbrown::HashSet;

const MAX_PLAYS: usize = 5;
const GRID_SIZE: usize = 9;

pub struct GameState {
    pub noughts: Vec<Sprite>,
    pub crosses: Vec<Sprite>,
    pub overlays: Vec<Sprite>,
    pub remaining_plays: HashSet<u8>,
    pub player_entries: HashSet<u8>,
    pub computer_entries: HashSet<u8>,
    pub level_winner: LevelWinner,
    pub level_over: bool,
    pub last_play_time: f32,
    pub turn_phase: TurnPhase,
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset_game(&mut self) -> Result<(), Error> {
        *self = Self::default();
        Ok(())
    }
}

impl Default for GameState {
    fn default() -> Self {
        let noughts = Vec::with_capacity(GRID_SIZE);
        let crosses = Vec::with_capacity(GRID_SIZE);
        let remaining_plays = HashSet::from([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let player_entries = HashSet::with_capacity(MAX_PLAYS);
        let computer_entries = HashSet::with_capacity(MAX_PLAYS);
        let level_winner = LevelWinner::Neither;
        let overlays = Vec::new();
        let level_over = false;
        let last_play_time = System::get().get_elapsed_time().unwrap();
        let turn_phase = TurnPhase::LevelStart { timestamp: last_play_time };

        Self {
            noughts,
            crosses,
            overlays,
            remaining_plays,
            player_entries,
            computer_entries,
            level_winner,
            level_over,
            last_play_time,
            turn_phase
        }
    }
}
