use bevy::{ecs::query::QueryFilter, platform::collections::HashSet, prelude::*};

use crate::chess::{board::Board, direction::Direction, moves::{CaptureGenerator, GenerateMovesEvent, Move, MoveGenerator, Moves, NormalMove}, piece::{Piece, PieceColor}, position::Position};

#[derive(Component, Clone)]
pub struct SlidingMoveGenerator(pub HashSet<Direction>);

impl SlidingMoveGenerator {
    fn generate<F: QueryFilter>(&self, commands: &mut Commands, moves: &mut Moves, board: &Board, position: Position, color: PieceColor, piece_colors: Query<&PieceColor, F>,
        allow_moves: bool, allow_captures: bool) {

        for &dir in &self.0 {
            let mut pos = position + dir;

            while board.is_empty(pos) {
                if allow_moves {
                    moves.insert(commands, pos, Move::Normal(NormalMove(position, pos)), false);
                }

                pos += dir;
            }

            if allow_captures && board.is_enemy(pos, color, piece_colors) {
                moves.insert(commands, pos, Move::Normal(NormalMove(position, pos)), true);
            }
        }
    }
}

pub fn generate_sliding_moves(_event: On<GenerateMovesEvent>, mut commands: Commands, board: Single<&Board>,
    pieces: Query<(&PieceColor, &Position, &mut Moves, &MoveGenerator<SlidingMoveGenerator>), With<Piece>>, piece_colors: Query<&PieceColor, With<Piece>>) {

    for (&color, &position, mut moves, MoveGenerator(move_gen)) in pieces {
        move_gen.generate(&mut commands, &mut moves, &board, position, color, piece_colors, true, false);
    }
}

pub fn generate_sliding_captures(_event: On<GenerateMovesEvent>, mut commands: Commands, board: Single<&Board>,
    pieces: Query<(&PieceColor, &Position, &mut Moves, &CaptureGenerator<SlidingMoveGenerator>), With<Piece>>, piece_colors: Query<&PieceColor, With<Piece>>) {

    for (&color, &position, mut moves, CaptureGenerator(move_gen)) in pieces {
        move_gen.generate(&mut commands, &mut moves, &board, position, color, piece_colors, false, true);
    }
}

pub fn generate_sliding_moves_and_captures(_event: On<GenerateMovesEvent>, mut commands: Commands, board: Single<&Board>,
    pieces: Query<(&PieceColor, &Position, &mut Moves, &SlidingMoveGenerator), With<Piece>>, piece_colors: Query<&PieceColor, With<Piece>>) {

    for (&color, &position, mut moves, move_gen) in pieces {
        move_gen.generate(&mut commands, &mut moves, &board, position, color, piece_colors, true, true);
    }
}
