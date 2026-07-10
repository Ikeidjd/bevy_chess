use bevy::prelude::*;

use crate::chess::{board::Board, direction::Direction, moves::{GenerateMovesEvent, HasMoved, Move, Moves, NormalMove}, piece::Piece, position::Position};

#[derive(Component, Clone, Copy)]
pub struct PawnMoveGenerator(pub Direction);

pub fn generate_pawn_moves(_event: On<GenerateMovesEvent>, mut commands: Commands, board: Single<&Board>,
    pieces: Query<(Entity, &Position, &mut Moves, &PawnMoveGenerator), With<Piece>>, has_moved: Query<(), With<HasMoved>>) {

    for (piece, &position, mut moves, move_gen) in pieces {
        let &PawnMoveGenerator(direction) = move_gen;

        let mut pos = position + direction;

        if !board.is_empty(pos) {
            continue;
        }

        moves.insert(&mut commands, pos, Move::Normal(NormalMove(position, pos)), false);

        if has_moved.get(piece).is_ok() {
            continue;
        }

        pos += direction;

        if board.is_empty(pos) {
            moves.insert(&mut commands, pos, Move::Normal(NormalMove(position, pos)), false);
        }
    }
}
