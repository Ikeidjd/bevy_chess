use bevy::{platform::collections::HashSet, prelude::*};

use crate::chess::{board::Board, direction::Direction, moves::{CaptureGenerator, GenerateMovesEvent, Move, MoveGenerator, Moves, NormalMove}, piece::{Piece, PieceColor, SelectedPiece}, position::Position};

#[derive(Component, Clone)]
pub struct SingleMoveGenerator(pub HashSet<Direction>);

pub fn generate_single_moves(_event: On<GenerateMovesEvent>, mut commands: Commands,
    board: Single<&Board>, mut piece: Single<(&Position, &mut Moves, &MoveGenerator<SingleMoveGenerator>), (With<Piece>, With<SelectedPiece>)>) {

    let (&position, ref mut moves, MoveGenerator(move_gen)) = *piece;

    for &dir in &move_gen.0 {
        let pos = position + dir;

        if board.is_empty(pos) {
            moves.insert(&mut commands, pos, Move::Normal(NormalMove(position, pos)), false);
        }
    }
}

pub fn generate_single_captures(_event: On<GenerateMovesEvent>, mut commands: Commands,
    board: Single<&Board>, mut piece: Single<(&PieceColor, &Position, &mut Moves, &CaptureGenerator<SingleMoveGenerator>), (With<Piece>, With<SelectedPiece>)>,
    piece_colors: Query<&PieceColor, With<Piece>>) {

    let (&color, &position, ref mut moves, CaptureGenerator(move_gen)) = *piece;

    for &dir in &move_gen.0 {
        let pos = position + dir;

        if board.is_enemy(pos, color, piece_colors) {
            moves.insert(&mut commands, pos, Move::Normal(NormalMove(position, pos)), true);
        }
    }
}

pub fn generate_single_moves_and_captures(_event: On<GenerateMovesEvent>, mut commands: Commands,
    board: Single<&Board>, mut piece: Single<(&PieceColor, &Position, &mut Moves, &SingleMoveGenerator), (With<Piece>, With<SelectedPiece>)>,
    piece_colors: Query<&PieceColor, With<Piece>>) {

    let (&color, &position, ref mut moves, move_gen) = *piece;

    for &dir in &move_gen.0 {
        let pos = position + dir;
        let empty = board.is_empty(pos);

        if empty || board.is_enemy(pos, color, piece_colors) {
            moves.insert(&mut commands, pos, Move::Normal(NormalMove(position, pos)), !empty);
        }
    }
}
