use bevy::{platform::collections::HashMap, prelude::*};

use crate::{chess::{board::{Board, BoardChanges}, markers::{CastleMarker, EnPassantMarker, MarkerBoard}, moves::animation::PieceAnimationStartedEvent, piece::{Piece, PieceDeselectedEvent}, position::Position}, layers};

#[derive(Component, Clone)]
pub (crate) struct Moves {
    pub (crate) positions: HashMap<Position, Move>,
    pub (crate) black_circles: Vec<Entity>,
    pub (crate) mmove: Option<Handle<Image>>,
    pub (crate) capture: Option<Handle<Image>>,
}

impl Moves {
    pub (crate) fn new(asset_server: &AssetServer) -> Self {
        Self {
            positions: HashMap::new(),
            black_circles: Vec::new(),
            mmove: Some(asset_server.load("move_indicator.png")),
            capture: Some(asset_server.load("capture_indicator.png")),
        }
    }

    pub (crate) fn new_no_move_indicators() -> Self {
        Self {
            positions: HashMap::new(),
            black_circles: Vec::new(),
            mmove: None,
            capture: None,
        }
    }

    pub (crate) fn insert(&mut self, commands: &mut Commands, position: Position, mmove: Move) {
        if MovePriority::get_priority(position, Some(mmove)) > MovePriority::get_priority(position, self.positions.get(&position).copied()) {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum MovePriority {
    None,
    NoCapture,
    NormalCapture,
    MarkerCapture,
}

impl MovePriority {
    fn get_priority(position: Position, mmove: Option<Move>) -> MovePriority {
        let Some(mmove) = mmove else {
            return MovePriority::None;
        };

        let Some(capture) = mmove.capture else {
            return MovePriority::NoCapture;
        };

        match capture == position {
            true => MovePriority::NormalCapture,
            false => MovePriority::MarkerCapture,
        }
    }
}

#[derive(Clone, Copy)]
pub (crate) struct NormalMove {
    pub (crate) from: Position,
    pub (crate) to: Position,
}

impl NormalMove {
    pub (crate) fn new(from: Position, to: Position) -> Self {
        Self {
            from,
            to,
        }
    }
}

#[derive(Clone, Copy)]
pub (crate) enum MoveType {
    Normal(NormalMove),
    DoublePawn(NormalMove),
    Castle(NormalMove, NormalMove),
}

#[derive(Clone, Copy)]
pub (crate) struct Move {
    pub (crate) capture: Option<Position>,
    pub (crate) move_type: MoveType,
}

#[derive(Component)]
pub (crate) struct HasMoved;

#[derive(Component)]
pub (crate) struct BlackCircle;

#[derive(Event)]
pub (crate) struct GenerateMovesEvent;

#[derive(Event, Clone)]
pub (crate) struct PieceMovedEvent {
    pub (crate) mmove: Move,
    pub (crate) is_real: bool, // Moves that are just being simulated to check for illegal moves aren't real; they shouldn't add a HasMoved component or start an animation
    pub (crate) advance_move: bool,
}

impl PieceMovedEvent {
    pub (crate) fn new(mmove: Move, is_real: bool, advance_move: bool) -> Self {
        Self {
            mmove,
            is_real,
            advance_move,
        }
    }
}

#[derive(Event)]
pub (crate) struct MoveFullyEndedEvent;

pub (crate) fn on_piece_moved(event: On<PieceMovedEvent>, mut commands: Commands, mut board: Single<(&mut Board, &mut BoardChanges)>, mut marker_board: Single<&mut MarkerBoard>,
    pieces: Query<Entity, With<Piece>>) {

    let (ref mut board, ref mut board_changes) = *board;

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

            let marker_position = Position::new((from.rank + to.rank) / 2, (from.file + to.file) / 2);

            let marker = commands.spawn((
                EnPassantMarker(board[from]),
                marker_position,
                Transform::from_xyz(0.0, 0.0, layers::MARKERS),
            )).id();

            marker_board.insert(marker);

            let mmove = Move {
                move_type: MoveType::Normal(normal_move),
                capture: event.mmove.capture,
            };

            commands.trigger(PieceMovedEvent::new(mmove, event.is_real, false));
        }
        MoveType::Castle(king_move, rook_move) => {
            let mut pos = king_move.from;
            let dir = (king_move.to - king_move.from).normalize();

            while pos != king_move.to {
                let marker = commands.spawn((
                    CastleMarker(board[king_move.from]),
                    pos,
                    Transform::from_xyz(0.0, 0.0, layers::MARKERS),
                )).id();

                marker_board.insert(marker);
                pos += dir;
            }

            let king_move = Move {
                move_type: MoveType::Normal(king_move),
                capture: event.mmove.capture,
            };

            let rook_move = Move {
                move_type: MoveType::Normal(rook_move),
                capture: event.mmove.capture,
            };

            commands.trigger(PieceMovedEvent::new(king_move, event.is_real, false));
            commands.trigger(PieceMovedEvent::new(rook_move, event.is_real, false));
        }
    }

    if event.advance_move {
        marker_board.advance_move();
    }

    commands.trigger(PieceDeselectedEvent);
}

pub (crate) fn on_move_fully_ended(_event: On<MoveFullyEndedEvent>, mut board_changes: Single<&mut BoardChanges, With<Board>>,) {
    board_changes.clear();
}
