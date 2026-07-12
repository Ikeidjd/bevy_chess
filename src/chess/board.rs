use std::ops::{Index, IndexMut};

use bevy::{ecs::query::QueryFilter, prelude::*};

use crate::{CursorWorldCoordinates, chess::{BOARD_LENGTH, moves::{HasMoved, Moves, PieceMovedEvent}, piece::{Piece, PieceColor, PieceDeselectedEvent, PieceSelectedEvent, SelectedPiece, StopFollowingCursorEvent}, position::Position}};

#[derive(Component, Clone)]
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

    pub fn do_move(&mut self, commands: &mut Commands, board_changes: &mut BoardChanges, piece: Entity, from: Position, to: Position, is_real: bool) {
        board_changes.changes.push((from, self[from]));
        board_changes.changes.push((to, self[to]));

        if !self.is_empty(to) {
            board_changes.to_despawn.push(self[to]);
            commands.entity(self[to]).insert((Captured, Visibility::Hidden));
        }

        self[to] = piece;
        self[from] = self.empty_piece;

        commands.entity(piece).insert(to);

        if is_real {
            commands.entity(piece).insert(HasMoved);
        }
    }

    pub fn restore_changes(&mut self, commands: &mut Commands, board_changes: &mut BoardChanges) {
        let BoardChanges { changes, .. } = std::mem::take(board_changes);

        for (position, piece) in changes.into_iter().rev() {
            self[position] = piece;
            commands.entity(piece).insert((position, Visibility::Visible)).remove::<Captured>();
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

#[derive(Component, Default)]
pub struct BoardChanges {
    pub changes: Vec<(Position, Entity)>,
    pub to_despawn: Vec<Entity>,
}

impl BoardChanges {
    pub fn clear(&mut self, commands: &mut Commands) {
        let Self { to_despawn, .. } = std::mem::take(self);

        for piece in to_despawn {
            commands.entity(piece).despawn();
        }
    }
}

#[derive(Component)]
pub struct Captured;

#[derive(Event)]
pub struct BoardPressedEvent(pub Position);

#[derive(Event)]
pub struct BoardReleasedEvent(pub Position);

#[derive(Event)]
pub struct RestoreBoardEvent;

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
                commands.trigger(PieceMovedEvent::new(mmove, true));
            }
            None => {
                commands.trigger(StopFollowingCursorEvent(selected_piece_entity));
            }
        }
    }
}

pub fn restore_board(_event: On<RestoreBoardEvent>, mut commands: Commands, mut board: Single<(&mut Board, &mut BoardChanges)>) {
    let (ref mut board, ref mut board_changes) = *board;
    board.restore_changes(&mut commands, board_changes);
}
