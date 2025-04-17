use crate::enums::LevelWinner::{Computer, Neither, User};
use crate::Quadrant::{
    BottomLeft, BottomMiddle, BottomRight, Center, CenterLeft, CenterRight, TopLeft, TopMiddle,
    TopRight,
};
use crate::SpriteType::{Background, Cross, GameOver, LevelOver, LevelStart, Nought, Player};
use crankstart::log_to_console;

/*
 All game logic assumes the following positions for TicTacToe squares:

     0 | 1 | 2
    --- --- ---
     3 | 4 | 5
    --- --- ---
     6 | 7 | 8
*/
#[derive(PartialEq)]
#[repr(u8)]
pub enum Quadrant {
    TopLeft = 0,
    TopMiddle = 1,
    TopRight = 2,
    CenterLeft = 3,
    Center = 4,
    CenterRight = 5,
    BottomLeft = 6,
    BottomMiddle = 7,
    BottomRight = 8,
}

impl Quadrant {
    // TODO: Replace with From/Into trait? https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
    pub fn from_location(x: f32, y: f32) -> Quadrant {
        match (x, y) {
            (60.0, 40.0) => TopLeft,
            (200.0, 40.0) => TopMiddle,
            (340.0, 40.0) => TopRight,
            (60.0, 120.0) => CenterLeft,
            (200.0, 120.0) => Center,
            (340.0, 120.0) => CenterRight,
            (60.0, 200.0) => BottomLeft,
            (200.0, 200.0) => BottomMiddle,
            (340.0, 200.0) => BottomRight,
            _ => {
                log_to_console!("Passed in unexpected coordinates ({:?}, {:?}), defaulting to center", x, y);
                Center
            }
        }
    }

    pub fn to_location(self) -> (f32, f32) {
        match self {
            TopLeft => (60.0, 40.0),
            TopMiddle => (200.0, 40.0),
            TopRight => (340.0, 40.0),
            CenterLeft => (60.0, 120.0),
            Center => (200.0, 120.0),
            CenterRight => (340.0, 120.0),
            BottomLeft => (60.0, 200.0),
            BottomMiddle => (200.0, 200.0),
            BottomRight => (340.0, 200.0),
        }
    }

    pub fn move_right(self) -> Quadrant {
        match self {
            TopLeft => TopMiddle,
            TopMiddle => TopRight,
            CenterLeft => Center,
            Center => CenterRight,
            BottomLeft => BottomMiddle,
            BottomMiddle => BottomRight,
            _ => self,
        }
    }

    pub fn move_left(self) -> Quadrant {
        match self {
            TopMiddle => TopLeft,
            TopRight => TopMiddle,
            Center => CenterLeft,
            CenterRight => Center,
            BottomMiddle => BottomLeft,
            BottomRight => BottomMiddle,
            _ => self,
        }
    }

    pub fn move_up(self) -> Quadrant {
        match self {
            CenterLeft => TopLeft,
            Center => TopMiddle,
            CenterRight => TopRight,
            BottomLeft => CenterLeft,
            BottomMiddle => Center,
            BottomRight => CenterRight,
            _ => self,
        }
    }

    pub fn move_down(self) -> Quadrant {
        match self {
            TopLeft => CenterLeft,
            TopMiddle => Center,
            TopRight => CenterRight,
            CenterLeft => BottomLeft,
            Center => BottomMiddle,
            CenterRight => BottomRight,
            _ => self,
        }
    }
}

impl From<u8> for Quadrant {
    fn from(value: u8) -> Self {
        let quadrant = match value {
            0 => TopLeft,
            1 => TopMiddle,
            2 => TopRight,
            3 => CenterLeft,
            4 => Center,
            5 => CenterRight,
            6 => BottomLeft,
            7 => BottomMiddle,
            _ => BottomRight,
        };
        quadrant
    }
}

#[repr(u8)]
pub enum SpriteType {
    Background = 0,
    Player = 1,
    Cross = 2,
    Nought = 3,
    LevelStart = 4,
    LevelOver = 5,
    GameOver = 6
}

impl From<u8> for SpriteType {
    fn from(tag: u8) -> Self {
        let sprite_type = match tag {
            0 => Background,
            1 => Player,
            2 => Cross,
            3 => Nought,
            4 => LevelStart,
            5 => LevelOver,
            _ => GameOver
        };
        sprite_type
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum LevelWinner {
    User = 0,
    Computer = 1,
    Neither = 2
}

impl From<u8> for LevelWinner {
    fn from(tag: u8) -> Self {
        let level_winner = match tag {
            0 => User,
            1 => Computer,
            _ => Neither
        };
        level_winner
    }
}

#[derive(Debug, PartialEq)]
pub enum LevelId {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

#[derive(Debug, PartialEq)]
pub enum TurnPhase {
    LevelStart { timestamp: f32 },
    PlayersTurn,
    ComputersTurn { timestamp: f32 },
    LevelOver { timestamp: f32 },
    GameOver { timestamp: f32 }
}