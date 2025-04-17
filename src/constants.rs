pub const CROSS_Z_INDEX: i16 = 10;
pub const NOUGHT_Z_INDEX: i16 = 10;

pub const PLAYER_Z_INDEX: i16 = 20;
pub const LEVEL_OVERLAY_INDEX: i16 = 30;

pub const INTERACTION_DELAY: f32 = 0.5;

pub const WINNING_COMBINATIONS: [[u8; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];