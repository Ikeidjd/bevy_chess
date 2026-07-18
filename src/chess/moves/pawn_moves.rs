use bevy::prelude::*;

use crate::chess::{board::Board, direction::Direction, moves::moves::{GenerateMovesEvent, HasMoved, Move, MoveType, Moves, NormalMove}, piece::Piece, position::Position};

#[derive(Component, Clone, Copy)]
pub (crate) struct DoublePawnMoveGenerator(pub (crate) Direction);

pub (crate) fn generate_pawn_moves(_event: On<GenerateMovesEvent>, mut commands: Commands, board: Single<&Board>,
    pieces: Query<(Entity, &Position, &mut Moves, &DoublePawnMoveGenerator), With<Piece>>, has_moved: Query<(), With<HasMoved>>) {

    for (piece, &position, mut moves, move_gen) in pieces {
        let &DoublePawnMoveGenerator(direction) = move_gen;

        let pos = position + direction * 2;

        if has_moved.get(piece).is_ok() || !board.is_empty(position + direction) || !board.is_empty(pos) {
            continue;
        }

        moves.insert(&mut commands, pos, Move {
            move_type: MoveType::DoublePawn(NormalMove::new(position, pos)),
            capture: None,
        });
    }
}
