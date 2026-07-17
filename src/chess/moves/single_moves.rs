use bevy::{ecs::query::QueryFilter, platform::collections::{HashMap, HashSet}, prelude::*};

use crate::chess::{board::Board, direction::Direction, moves::{move_generator::{MoveGenerator, move_generator_plugin}, moves::{Move, MoveType, Moves, NormalMove}}, piece::PieceColor, position::Position};

#[derive(Component, Clone)]
pub struct SingleMoveGenerator(pub HashSet<Direction>);

impl MoveGenerator for SingleMoveGenerator {
    fn generate<F: QueryFilter>(&self, commands: &mut Commands, moves: &mut Moves, board: &Board, position: Position, color: PieceColor, piece_colors: Query<&PieceColor, F>,
        allow_moves: bool, allow_captures: bool) {

        for &dir in &self.0 {
            let pos = position + dir;

            if allow_moves && board.is_empty(pos) {
                moves.insert(commands, pos, Move {
                    move_type: MoveType::Normal(NormalMove::new(position, pos)),
                    capture: None,
                });
            }

            if allow_captures && board.is_enemy(pos, color, piece_colors) {
                moves.insert(commands, pos, Move {
                    move_type: MoveType::Normal(NormalMove::new(position, pos)),
                    capture: Some(pos),
                });
            }
        }
    }

    fn generate_marker_captures<F: QueryFilter>(&self, commands: &mut Commands, moves: &mut Moves, board: &Board, position: Position, color: PieceColor,
        piece_colors: Query<&PieceColor, F>, marker_to_piece: HashMap<Position, Position>) {

        for &dir in &self.0 {
            let pos = position + dir;

            if let Some(&piece_pos) = marker_to_piece.get(&pos) && board.is_enemy(piece_pos, color, piece_colors) {
                moves.insert(commands, pos, Move {
                    move_type: MoveType::Normal(NormalMove::new(position, pos)),
                    capture: Some(piece_pos),
                });
            }
        }
    }
}

move_generator_plugin!(SingleMoveGeneratorPlugin, SingleMoveGenerator);
