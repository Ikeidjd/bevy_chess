use bevy::{platform::collections::HashMap, prelude::*};

use crate::{chess::{board::{Board, BoardChanges}, moves::{animation::PieceAnimationStartedEvent, move_generator::PieceMarkerRequire, pawn_moves::EnPassantMarker}, piece::{Piece, PieceDeselectedEvent}, position::Position}, layers};

#[derive(Component, Clone)]
pub struct Moves {
    pub positions: HashMap<Position, Move>,
    pub black_circles: Vec<Entity>,
    pub mmove: Option<Handle<Image>>,
    pub capture: Option<Handle<Image>>,
}

impl Moves {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            positions: HashMap::new(),
            black_circles: Vec::new(),
            mmove: Some(asset_server.load("move_indicator.png")),
            capture: Some(asset_server.load("capture_indicator.png")),
        }
    }

    pub fn new_no_move_indicators() -> Self {
        Self {
            positions: HashMap::new(),
            black_circles: Vec::new(),
            mmove: None,
            capture: None,
        }
    }

    pub fn insert(&mut self, commands: &mut Commands, position: Position, mmove: Move) {
        if !self.positions.contains_key(&position) {
            self.positions.insert(position, mmove);

            let image = match mmove.capture {
                Some(_) => self.capture.clone(),
                None => self.mmove.clone(),
            };

            let image = match image {
                Some(image) => image,
                None => return,
            };

            self.black_circles.push(commands.spawn((
                BlackCircle,
                position,
                Sprite::from(image),
                Transform::from_xyz(0.0, 0.0, layers::BLACK_CIRCLES),
            )).id());
        }
    }
}

#[derive(Clone, Copy)]
pub struct NormalMove {
    pub from: Position,
    pub to: Position,
}

impl NormalMove {
    pub fn new(from: Position, to: Position) -> Self {
        Self {
            from,
            to,
        }
    }
}

#[derive(Clone, Copy)]
pub enum MoveType {
    Normal(NormalMove),
    DoublePawn(NormalMove),
    Castle(NormalMove, NormalMove),
}

#[derive(Clone, Copy)]
pub struct Move {
    pub capture: Option<Position>,
    pub move_type: MoveType,
}

#[derive(Component)]
pub struct HasMoved;

#[derive(Component)]
pub struct BlackCircle;

#[derive(Event)]
pub struct GenerateMovesEvent;

#[derive(Event, Clone)]
pub struct PieceMovedEvent {
    pub mmove: Move,
    pub is_real: bool, // Moves that are just being simulated to check for illegal moves aren't real; they shouldn't add a HasMoved component or start an animation
}

impl PieceMovedEvent {
    pub fn new(mmove: Move, is_real: bool) -> Self {
        Self {
            mmove,
            is_real,
        }
    }
}

#[derive(Event)]
pub struct MoveFullyEndedEvent;

pub fn on_piece_moved(event: On<PieceMovedEvent>, mut commands: Commands, mut board: Single<(Entity, &mut Board, &mut BoardChanges)>, pieces: Query<Entity, With<Piece>>) {
    let (board_entity, ref mut board, ref mut board_changes) = *board;

    match event.mmove.move_type {
        MoveType::Normal(NormalMove { from, to }) => {
            let piece = pieces.get(board[from]).unwrap();
            board.do_move(&mut commands, board_changes, piece, from, to, event.mmove.capture, event.is_real);

            if event.is_real {
                commands.trigger(PieceAnimationStartedEvent(piece, from, to));
            }
        }
        MoveType::DoublePawn(normal_move) => {
            let NormalMove { from, to } = normal_move;

            let marker = commands.spawn((
                EnPassantMarker(board[from]),
                Position::new((from.rank + to.rank) / 2, (from.file + to.file) / 2),
            )).id();

            commands.entity(board_entity).add_child(marker);

            let mmove = Move {
                move_type: MoveType::Normal(normal_move),
                capture: event.mmove.capture,
            };

            commands.trigger(PieceMovedEvent::new(mmove, event.is_real));
        }
        MoveType::Castle(normal_move_a, normal_move_b) => {
            let move_a = Move {
                move_type: MoveType::Normal(normal_move_a),
                capture: event.mmove.capture,
            };

            let move_b = Move {
                move_type: MoveType::Normal(normal_move_b),
                capture: event.mmove.capture,
            };

            commands.trigger(PieceMovedEvent::new(move_a, event.is_real));
            commands.trigger(PieceMovedEvent::new(move_b, event.is_real));
        }
    }

    commands.trigger(PieceDeselectedEvent);
}

pub fn on_move_fully_ended(_event: On<MoveFullyEndedEvent>, mut commands: Commands, mut board_changes: Single<&mut BoardChanges, With<Board>>, markers: Query<(Entity, &mut PieceMarkerRequire)>) {
    board_changes.clear(&mut commands);

    for (entity, mut marker) in markers {
        match marker.old {
            true => commands.entity(entity).despawn(),
            false => marker.old = true,
        }
    }
}
