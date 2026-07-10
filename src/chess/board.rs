use std::ops::{Index, IndexMut};

use bevy::{ecs::query::QueryFilter, prelude::*};

use crate::{CursorWorldCoordinates, chess::{BOARD_LENGTH, moves::{Moves, PieceMovedEvent}, piece::{Piece, PieceColor, PieceDeselectedEvent, PieceSelectedEvent, SelectedPiece, StopFollowingCursorEvent}, position::Position}};

#[derive(Component)]
pub struct Board {
    pub pieces: [[Entity; BOARD_LENGTH.x]; BOARD_LENGTH.y],
    pub empty_piece: Entity,
}

impl Board {
    pub fn new(empty_piece: Entity) -> Self {
        Self {
            pieces: [[empty_piece; BOARD_LENGTH.x]; BOARD_LENGTH.y],
            empty_piece: empty_piece,
        }
    }

    pub fn is_in_bounds(&self, position: Position) -> bool {
        position.rank >= 0 && position.rank < BOARD_LENGTH.x as isize && position.file >= 0 && position.file < BOARD_LENGTH.y as isize
    }

    pub fn is_empty(&self, position: Position) -> bool {
        self.is_in_bounds(position) && self[position] == self.empty_piece
    }

    pub fn is_enemy<F: QueryFilter>(&self, position: Position, color: PieceColor, piece_colors: Query<&PieceColor, F>) -> bool {
        self.is_in_bounds(position) && match piece_colors.get(self[position]) {
            Ok(&target_color) => color != target_color,
            Err(_) => false,
        }
    }
}

impl Index<Position> for Board {
    type Output = Entity;

    fn index(&self, position: Position) -> &Self::Output {
        &self.pieces[position.rank as usize][position.file as usize]
    }
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        &mut self.pieces[position.rank as usize][position.file as usize]
    }
}

#[derive(Event)]
pub struct BoardPressedEvent(Position);

#[derive(Event)]
pub struct BoardReleasedEvent(Position);

pub fn check_board_clicked(mut commands: Commands, input: Res<ButtonInput<MouseButton>>, cursor: Res<CursorWorldCoordinates>) {
    let cursor = Position::from_translation(cursor.0);

    if input.just_pressed(MouseButton::Left) {
        commands.trigger(BoardPressedEvent(cursor));
    } else if input.just_released(MouseButton::Left) {
        commands.trigger(BoardReleasedEvent(cursor));
    }
}

pub fn on_board_pressed(event: On<BoardPressedEvent>, mut commands: Commands, board: Single<&Board>, selected_piece: Query<&Moves, (With<Piece>, With<SelectedPiece>)>) {
    if let Ok(moves) = selected_piece.single() && moves.positions.contains_key(&event.0) {
        return;
    }

    if board.is_in_bounds(event.0) {
        commands.trigger(PieceDeselectedEvent);
        commands.trigger(PieceSelectedEvent(board[event.0]));
    }
}

pub fn on_board_released(event: On<BoardReleasedEvent>, mut commands: Commands, selected_piece: Query<(Entity, &Moves), (With<Piece>, With<SelectedPiece>)>) {
    if let Ok((selected_piece_entity, moves)) = selected_piece.single() {
        match moves.positions.get(&event.0) {
            Some(&mmove) => {
                commands.trigger(PieceMovedEvent(mmove));
            }
            None => {
                commands.trigger(StopFollowingCursorEvent(selected_piece_entity));
            }
        }
    }
}
