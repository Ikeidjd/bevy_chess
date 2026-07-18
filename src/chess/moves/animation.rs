use bevy::prelude::*;

use crate::chess::{ChessState, moves::promotion::AttemptPromotionEvent, piece::PieceFollowsCursor, position::Position};

#[derive(Component)]
pub (crate) struct PieceAnimation {
    pub (crate) start: Vec2,
    pub (crate) end: Vec2,
    pub (crate) progress: f32,
}

impl PieceAnimation {
    pub (crate) const SPEED: f32 = 4.0;
}

#[derive(Event)]
pub (crate) struct PieceAnimationStartedEvent(pub (crate) Entity, pub (crate) Position, pub (crate) Position);

pub (crate) fn on_piece_animation_started(event: On<PieceAnimationStartedEvent>, mut commands: Commands, mut next_state: ResMut<NextState<ChessState>>,
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

pub (crate) fn update_piece_animations(mut commands: Commands, mut next_state: ResMut<NextState<ChessState>>, time: Res<Time>, pieces: Query<(Entity, &mut PieceAnimation, &mut Transform)>) {
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
