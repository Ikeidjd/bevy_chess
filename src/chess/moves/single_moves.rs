use bevy::{platform::collections::HashSet, prelude::*};

use crate::chess::{board::Board, direction::Direction, moves::{GenerateMovesEvent, Move, Moves, NormalMove}, piece::{Piece, PieceColor, SelectedPiece}, position::Position};

#[derive(Component, Clone)]
pub struct SingleMoveGenerator(pub HashSet<Direction>);

pub fn generate_single_moves(_event: On<GenerateMovesEvent>, mut commands: Commands,
    board: Single<&Board>, mut piece: Single<(&PieceColor, &Position, &mut Moves, &SingleMoveGenerator), (With<Piece>, With<SelectedPiece>)>,
    piece_colors: Query<&PieceColor, With<Piece>>) {

    let (&color, &position, ref mut moves, move_gen) = *piece;

    for &dir in &move_gen.0 {
        let pos = position + dir;

        if board.is_empty(pos) || board.is_enemy(pos, color, piece_colors) {
            moves.insert(&mut commands, pos, Move::Normal(NormalMove(position, pos)));
        }
    }
}
