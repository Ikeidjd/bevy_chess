use bevy::{platform::collections::HashMap, prelude::*};

use crate::{chess::{ChessState, board::{Board, BoardChanges}, moves::promotion::AttemptPromotionEvent, piece::{Piece, PieceDeselectedEvent, PieceFollowsCursor}, position::Position}, layers};

pub mod single_moves;
pub mod sliding_moves;
pub mod castling_moves;
pub mod pawn_moves;
pub mod promotion;
pub mod checks;

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

    pub fn insert(&mut self, commands: &mut Commands, position: Position, mmove: Move, is_capture: bool) {
        if !self.positions.contains_key(&position) {
            self.positions.insert(position, mmove);

            let image = match is_capture {
                true => self.capture.clone(),
                false => self.mmove.clone(),
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
pub struct NormalMove(pub Position, pub Position);

#[derive(Clone, Copy)]
pub enum Move {
    Normal(NormalMove),
    Castle(NormalMove, NormalMove),
}

#[derive(Component)]
pub struct MoveGenerator<T: Component>(pub T);

#[derive(Component)]
pub struct CaptureGenerator<T: Component>(pub T);

#[derive(Component)]
pub struct HasMoved;

#[derive(Component)]
pub struct PieceAnimation {
    pub start: Vec2,
    pub end: Vec2,
    pub progress: f32,
}

impl PieceAnimation {
    pub const SPEED: f32 = 4.0;
}

#[derive(Component)]
pub struct BlackCircle;

#[derive(Event)]
pub struct GenerateMovesEvent;

#[derive(Event, Clone)]
pub struct PieceMovedEvent {
    pub mmove: Move,
    pub is_real: bool, // Moves done to check for illegal moves aren't real; they shouldn't add a HasMoved component or start an animation
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
pub struct PieceAnimationStartedEvent(pub Entity, pub Position, pub Position);

#[derive(Event)]
pub struct MoveFullyEndedEvent;

pub fn on_piece_moved(event: On<PieceMovedEvent>, mut commands: Commands, mut board: Single<(&mut Board, &mut BoardChanges)>, mut pieces: Query<Entity, With<Piece>>) {
    match event.mmove {
        Move::Normal(NormalMove(from, to)) => {
            let (ref mut board, ref mut board_changes) = *board;
            let piece = pieces.get_mut(board[from]).unwrap();
            board.do_move(&mut commands, board_changes, piece, from, to, event.is_real);

            if event.is_real {
                commands.trigger(PieceAnimationStartedEvent(piece, from, to));
            }
        }
        Move::Castle(normal_move_a, normal_move_b) => {
            commands.trigger(PieceMovedEvent::new(Move::Normal(normal_move_a), event.is_real));
            commands.trigger(PieceMovedEvent::new(Move::Normal(normal_move_b), event.is_real));
        }
    }

    commands.trigger(PieceDeselectedEvent);
}

pub fn on_piece_animation_started(event: On<PieceAnimationStartedEvent>, mut commands: Commands, mut next_state: ResMut<NextState<ChessState>>,
    cursor_followers: Query<(), With<PieceFollowsCursor>>) {

    let PieceAnimationStartedEvent(piece, from, to) = *event;

    let progress = match cursor_followers.get(piece) {
        Ok(_) => 1.0,
        Err(_) => 0.0,
    };

    commands.entity(piece).insert(PieceAnimation {
        start: from.to_translation(),
        end: to.to_translation(),
        progress,
    });

    next_state.set(ChessState::PieceAnimation);
}

pub fn update_piece_animations(mut commands: Commands, mut next_state: ResMut<NextState<ChessState>>, time: Res<Time>, pieces: Query<(Entity, &mut PieceAnimation, &mut Transform)>) {
    for (piece, mut animation, mut transform) in pieces {
        animation.progress += PieceAnimation::SPEED * time.delta_secs();
        animation.progress = animation.progress.clamp(0.0, 1.0);

        if animation.progress == 1.0 {
            next_state.set(ChessState::Main);
            commands.trigger(AttemptPromotionEvent(piece));
            commands.entity(piece).remove::<PieceAnimation>();
        }

        let vec = animation.start.lerp(animation.end, animation.progress);

        transform.translation.x = vec.x;
        transform.translation.y = vec.y;
    }
}

pub fn on_move_fully_ended(_event: On<MoveFullyEndedEvent>, mut commands: Commands, mut board_changes: Single<&mut BoardChanges, With<Board>>) {
    board_changes.clear(&mut commands);
}
