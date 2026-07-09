use bevy::prelude::*;

use crate::{CursorWorldCoordinates, chess::{PIECE_SIZE, moves::{GenerateMovesEvent, Moves}, position::Position}};

#[derive(Component)]
pub struct Piece;

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Component)]
pub struct SelectedPiece { pub yellow_square: Entity }

#[derive(Component)]
pub struct EmptyPiece;

#[derive(Component)]
#[require(Sprite::from_color(Color::srgba(1.0, 1.0, 0.0, 0.5), vec2(PIECE_SIZE, PIECE_SIZE)))]
pub struct YellowSquare;

#[derive(Component)]
pub struct PieceFollowsCursor;

#[derive(Event)]
pub struct PieceSelectedEvent(pub Entity);

#[derive(Event)]
pub struct PieceDeselectedEvent;

pub fn on_piece_deselected(_event: On<PieceDeselectedEvent>, mut commands: Commands, selected_piece: Single<(Entity, &SelectedPiece, &Moves), With<Piece>>) {
    let (selected_piece_entity, selected_piece, moves) = *selected_piece;

    commands.entity(selected_piece.yellow_square).despawn();

    for &black_circle in &moves.black_circles {
        commands.entity(black_circle).despawn();
    }

    commands.entity(selected_piece_entity).remove::<(SelectedPiece, PieceFollowsCursor, Moves)>();
}

pub fn on_piece_selected(event: On<PieceSelectedEvent>, mut commands: Commands, asset_server: Res<AssetServer>, pieces: Query<(Entity, &Position), With<Piece>>) {
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
        PieceFollowsCursor,
        Moves::new(&asset_server),
    ));

    commands.trigger(GenerateMovesEvent);
}

pub fn piece_follow_cursor(cursor: Res<CursorWorldCoordinates>, mut piece: Single<&mut Transform, With<PieceFollowsCursor>>) {
    piece.translation.x = cursor.0.x;
    piece.translation.y = cursor.0.y;
}
