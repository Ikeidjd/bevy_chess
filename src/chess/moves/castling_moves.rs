use bevy::prelude::*;

use crate::chess::{board::Board, moves::{GenerateMovesEvent, Move, Moves, NormalMove}, piece::{Piece, PieceColor, SelectedPiece}, position::Position};

#[derive(Component)]
pub struct CastleTop;

#[derive(Component)]
pub struct CastleBottom;

pub fn generate_castling_moves(_event: On<GenerateMovesEvent>, mut commands: Commands, board: Single<&Board>,
    mut castle_top: Single<(&PieceColor, &Position, &mut Moves), (With<Piece>, With<SelectedPiece>, With<CastleTop>)>,
    castle_bottoms: Query<(Entity, &PieceColor, &Position), With<CastleBottom>>) {

    let (color, &position, ref mut moves) = *castle_top;

    for (bottom, bottom_color, &bottom_position) in castle_bottoms {
        if color != bottom_color {
            continue;
        }

        let dir = (bottom_position - position).normalize();
        let mut pos = position + dir;

        while board.is_empty(pos) {
            pos += dir;
        }

        if board.is_in_bounds(pos) && board[pos] == bottom {
            moves.insert(&mut commands, position + dir * 2, Move::Castle(NormalMove(position, position + dir * 2), NormalMove(bottom_position, position + dir)), false);
        }
    }
}
