use bevy::{platform::collections::HashSet, prelude::*};

use crate::chess::{board::Board, direction::Direction, moves::{CaptureGenerator, GenerateMovesEvent, Move, MoveGenerator, Moves, NormalMove}, piece::{Piece, PieceColor, SelectedPiece}, position::Position};

#[derive(Component, Clone)]
pub struct SlidingMoveGenerator(pub HashSet<Direction>);

pub fn generate_sliding_moves(_event: On<GenerateMovesEvent>, mut commands: Commands,
    board: Single<&Board>, mut piece: Single<(&Position, &mut Moves, &MoveGenerator<SlidingMoveGenerator>), (With<Piece>, With<SelectedPiece>)>) {

    let (&position, ref mut moves, MoveGenerator(move_gen)) = *piece;

    for &dir in &move_gen.0 {
        let mut pos = position + dir;

        while board.is_empty(pos) {
            moves.insert(&mut commands, pos, Move::Normal(NormalMove(position, pos)), false);
            pos += dir;
        }
    }
}

pub fn generate_sliding_captures(_event: On<GenerateMovesEvent>, mut commands: Commands,
    board: Single<&Board>, mut piece: Single<(&PieceColor, &Position, &mut Moves, &CaptureGenerator<SlidingMoveGenerator>), (With<Piece>, With<SelectedPiece>)>,
    piece_colors: Query<&PieceColor, With<Piece>>) {

    let (&color, &position, ref mut moves, CaptureGenerator(move_gen)) = *piece;

    for &dir in &move_gen.0 {
        let mut pos = position + dir;

        while board.is_empty(pos) {
            pos += dir;
        }

        if board.is_enemy(pos, color, piece_colors) {
            moves.insert(&mut commands, pos, Move::Normal(NormalMove(position, pos)), true);
        }
    }
}

pub fn generate_sliding_moves_and_captures(_event: On<GenerateMovesEvent>, mut commands: Commands,
    board: Single<&Board>, mut piece: Single<(&PieceColor, &Position, &mut Moves, &SlidingMoveGenerator), (With<Piece>, With<SelectedPiece>)>,
    piece_colors: Query<&PieceColor, With<Piece>>) {

    let (&color, &position, ref mut moves, move_gen) = *piece;

    for &dir in &move_gen.0 {
        let mut pos = position + dir;

        while board.is_empty(pos) {
            moves.insert(&mut commands, pos, Move::Normal(NormalMove(position, pos)), false);
            pos += dir;
        }

        if board.is_enemy(pos, color, piece_colors) {
            moves.insert(&mut commands, pos, Move::Normal(NormalMove(position, pos)), true);
        }
    }
}
