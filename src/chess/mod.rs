use bevy::{math::{USizeVec2, usizevec2}, prelude::*};

use crate::MainState;

pub mod plugin;
mod position;
mod direction;
mod preset_pieces;
mod setup;
mod board;
mod piece;
mod markers;
mod moves;

pub const PIECE_SIZE: f32 = 48.0;
pub const BOARD_LENGTH: USizeVec2 = usizevec2(8, 8);

#[derive(SubStates, Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
#[source(MainState = MainState::Chess)]
enum ChessState {
    #[default]
    Main,
    PieceAnimation,
    Promotion,
}
