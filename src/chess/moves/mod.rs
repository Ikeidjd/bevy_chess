use bevy::{platform::collections::HashMap, prelude::*};

use crate::chess::{ChessState, board::Board, piece::{Piece, PieceDeselectedEvent, PieceFollowsCursor}, position::Position};

pub mod single_moves;
pub mod sliding_moves;
pub mod castling_moves;
pub mod pawn_moves;

#[derive(Component)]
pub struct Moves {
    pub positions: HashMap<Position, Move>,
    pub black_circles: Vec<Entity>,
    pub mmove: Handle<Image>,
    pub capture: Handle<Image>,
}

impl Moves {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            positions: HashMap::new(),
            black_circles: Vec::new(),
            mmove: asset_server.load("move_indicator.png"),
            capture: asset_server.load("capture_indicator.png"),
        }
    }

    pub fn insert(&mut self, commands: &mut Commands, position: Position, mmove: Move, is_capture: bool) {
        if !self.positions.contains_key(&position) {
            self.positions.insert(position, mmove);

            self.black_circles.push(commands.spawn((
                BlackCircle,
                position,
                Sprite::from(match is_capture {
                    true => self.capture.clone(),
                    false => self.mmove.clone(),
                }),
                Transform::from_xyz(0.0, 0.0, 0.5),
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
pub struct PieceMovedEvent(pub Move);

#[derive(Event)]
pub struct PieceAnimationStartedEvent(pub Entity, pub Position, pub Position);

pub fn on_piece_moved(event: On<PieceMovedEvent>, mut commands: Commands, mut board: Single<&mut Board>, mut pieces: Query<(Entity, &mut Position), With<Piece>>) {
    match event.0 {
        Move::Normal(NormalMove(from, to)) => {
            let (piece, mut piece_position) = pieces.get_mut(board[from]).unwrap();

            commands.trigger(PieceAnimationStartedEvent(piece, from, to));

            if !board.is_empty(to) {
                commands.entity(board[to]).despawn();
            }

            board[from] = board.empty_piece;
            board[to] = piece;

            commands.entity(piece).insert(HasMoved);
            *piece_position = to;
        }
        Move::Castle(normal_move_a, normal_move_b) => {
            commands.trigger(PieceMovedEvent(Move::Normal(normal_move_a)));
            commands.trigger(PieceMovedEvent(Move::Normal(normal_move_b)));
        }
    }

    commands.trigger(PieceDeselectedEvent);
}

pub fn on_piece_animation_started(event: On<PieceAnimationStartedEvent>, mut commands: Commands, mut next_state: ResMut<NextState<ChessState>>,
    cursor_followers: Query<(), With<PieceFollowsCursor>>) {

    let PieceAnimationStartedEvent(piece, from, to) = *event;

    if cursor_followers.get(piece).is_err() {
        commands.entity(piece).insert(PieceAnimation {
            start: from.to_translation(),
            end: to.to_translation(),
            progress: 0.0
        });

        next_state.set(ChessState::PieceAnimation);
    }
}

pub fn update_piece_animations(mut commands: Commands, mut next_state: ResMut<NextState<ChessState>>, time: Res<Time>, pieces: Query<(Entity, &mut PieceAnimation, &mut Transform)>) {
    if pieces.is_empty() {
        next_state.set(ChessState::Main);
    }

    for (piece, mut animation, mut transform) in pieces {
        animation.progress += PieceAnimation::SPEED * time.delta_secs();
        animation.progress = animation.progress.clamp(0.0, 1.0);

        if animation.progress == 1.0 {
            commands.entity(piece).remove::<PieceAnimation>();
        }

        let vec = animation.start.lerp(animation.end, animation.progress);

        transform.translation.x = vec.x;
        transform.translation.y = vec.y;
    }
}
