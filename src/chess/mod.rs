use bevy::{math::{USizeVec2, usizevec2}, prelude::*};

use crate::MainState;

pub (crate) mod plugin;
mod position;
mod direction;
mod preset_pieces;
mod setup;
mod board;
mod piece;
mod markers;
mod moves;

pub (crate) const PIECE_SIZE: f32 = 48.0;
pub (crate) const BOARD_LENGTH: USizeVec2 = usizevec2(8, 8);

#[derive(SubStates, Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
#[source(MainState = MainState::Chess)]
enum ChessState {
    #[default]
    Main,
    PieceAnimation,
    Promotion,
}
