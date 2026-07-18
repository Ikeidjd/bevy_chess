use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{CursorWorldCoordinates, chess::{PIECE_SIZE, markers::CastleMarker, moves::{checks::CheckIllegalMovesEvent, move_generator::CapturesMarkersAlways, moves::{GenerateMovesEvent, Moves}}, position::Position}};

#[derive(Component)]
#[require(CapturesMarkersAlways::<CastleMarker>(PhantomData))]
pub (crate) struct Piece;

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub (crate) enum PieceColor {
    White,
    Black,
}

#[derive(Component)]
pub (crate) struct SelectedPiece { pub (crate) yellow_square: Entity }

#[derive(Component)]
pub (crate) struct EmptyPiece;

#[derive(Component)]
#[require(Sprite::from_color(Color::srgba(1.0, 1.0, 0.0, 0.5), vec2(PIECE_SIZE, PIECE_SIZE)))]
pub (crate) struct YellowSquare;

#[derive(Component)]
pub (crate) struct PieceFollowsCursor;

#[derive(Event)]
pub (crate) struct PieceSelectedEvent(pub (crate) Entity);

#[derive(Event)]
pub (crate) struct PieceDeselectedEvent;

#[derive(Event)]
pub (crate) struct StartFollowingCursorEvent(pub (crate) Entity);

#[derive(Event)]
pub (crate) struct StopFollowingCursorEvent(pub (crate) Entity);

pub (crate) fn on_piece_deselected(_event: On<PieceDeselectedEvent>, mut commands: Commands, selected_piece: Single<(Entity, &SelectedPiece, &Moves), With<Piece>>) {
    let (selected_piece_entity, selected_piece, moves) = *selected_piece;

    commands.entity(selected_piece.yellow_square).despawn();

    for &black_circle in &moves.black_circles {
        commands.entity(black_circle).despawn();
    }

    commands.entity(selected_piece_entity).remove::<(SelectedPiece, Moves)>();
    commands.trigger(StopFollowingCursorEvent(selected_piece_entity));
}

pub (crate) fn on_piece_selected(event: On<PieceSelectedEvent>, mut commands: Commands, pieces: Query<(Entity, &Position), With<Piece>>) {
    let (piece, &position) = match pieces.get(event.0) {
        Ok(piece) => piece,
        Err(_) => return,
    };

    let yellow_square = commands.spawn((
        YellowSquare,
        position,
    )).id();

    commands.entity(piece).insert((
        SelectedPiece { yellow_square },
        Moves::new_no_move_indicators(), // When illegal moves are checked, the ones that aren't illegal get added to a new Moves component with move indicators
    ));

    commands.trigger(StartFollowingCursorEvent(piece));
    commands.trigger(GenerateMovesEvent);
    commands.trigger(CheckIllegalMovesEvent);
}

pub (crate) fn piece_follow_cursor(cursor: Res<CursorWorldCoordinates>, mut piece: Single<&mut Transform, With<PieceFollowsCursor>>) {
    piece.translation.x = cursor.0.x;
    piece.translation.y = cursor.0.y;
}

pub (crate) fn start_following_cursor(event: On<StartFollowingCursorEvent>, mut commands: Commands, mut pieces: Query<&mut Transform, Without<PieceFollowsCursor>>) {
    if let Ok(mut transform) = pieces.get_mut(event.0) {
        transform.translation.z += 0.1;
        commands.entity(event.0).insert(PieceFollowsCursor);
    }
}

pub (crate) fn stop_following_cursor(event: On<StopFollowingCursorEvent>, mut commands: Commands, mut pieces: Query<&mut Transform, With<PieceFollowsCursor>>) {
    if let Ok(mut transform) = pieces.get_mut(event.0) {
        transform.translation.z -= 0.1;
        commands.entity(event.0).remove::<PieceFollowsCursor>();
    }
}
