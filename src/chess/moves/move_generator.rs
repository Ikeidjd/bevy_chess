use std::marker::PhantomData;

use bevy::{ecs::query::QueryFilter, platform::collections::HashMap, prelude::*};

use crate::chess::{board::Board, moves::moves::{GenerateMovesEvent, Moves}, piece::{Piece, PieceColor}, position::Position};

pub trait MoveGenerator {
    fn generate<F: QueryFilter>(&self, commands: &mut Commands, moves: &mut Moves, board: &Board, position: Position, color: PieceColor, piece_colors: Query<&PieceColor, F>,
        allow_moves: bool, allow_captures: bool);

    fn generate_marker_captures<F: QueryFilter>(&self, commands: &mut Commands, moves: &mut Moves, board: &Board, position: Position, color: PieceColor,
        piece_colors: Query<&PieceColor, F>, marker_to_piece: HashMap<Position, Position>);

    fn get_marker_to_piece<Marker: PieceMarker + Component + Copy, F: QueryFilter>(markers: Query<(&Marker, &Position), F>,
        piece_positions: Query<&Position, With<Piece>>) -> HashMap<Position, Position> {

        markers.iter().map(|(&marker, &position)| (position, piece_positions.get(marker.get_entity()).unwrap().clone())).collect()
    }
}


// A PieceMarker is used to signal that a piece can be captured from a position other than its own
// This is used for double pawn moves (en passant) and castling (the king can't move out of or through check, i.e., it can be captured even though that is not its position)
pub trait PieceMarker {
    fn get_entity(&self) -> Entity;
}

// Used for despawning markers
// Make every component that implements PieceMarker #require this or bad things will happen
#[derive(Component, Default)]
pub struct PieceMarkerRequire {
    pub old: bool,
}

#[derive(Component)]
pub struct MoveOnly<MoveGen: MoveGenerator + Component>(pub MoveGen);

#[derive(Component)]
pub struct CaptureOnly<MoveGen: MoveGenerator + Component>(pub MoveGen);

#[derive(Component)]
pub struct CaptureMarker<MoveGen: MoveGenerator + Component, Marker: PieceMarker + Component>(pub MoveGen, pub PhantomData<Marker>);

pub fn generate_moves<MoveGen: MoveGenerator + Component>(_event: On<GenerateMovesEvent>, mut commands: Commands, board: Single<&Board>,
    pieces: Query<(&PieceColor, &Position, &mut Moves, &MoveOnly<MoveGen>), With<Piece>>, piece_colors: Query<&PieceColor, With<Piece>>) {

    for (&color, &position, mut moves, MoveOnly(move_gen)) in pieces {
        move_gen.generate(&mut commands, &mut moves, &board, position, color, piece_colors, true, false);
    }
}

pub fn generate_captures<MoveGen: MoveGenerator + Component>(_event: On<GenerateMovesEvent>, mut commands: Commands, board: Single<&Board>,
    pieces: Query<(&PieceColor, &Position, &mut Moves, &CaptureOnly<MoveGen>), With<Piece>>, piece_colors: Query<&PieceColor, With<Piece>>) {

    for (&color, &position, mut moves, CaptureOnly(move_gen)) in pieces {
        move_gen.generate(&mut commands, &mut moves, &board, position, color, piece_colors, false, true);
    }
}

pub fn generate_moves_and_captures<MoveGen: MoveGenerator + Component>(_event: On<GenerateMovesEvent>, mut commands: Commands, board: Single<&Board>,
    pieces: Query<(&PieceColor, &Position, &mut Moves, &MoveGen), With<Piece>>, piece_colors: Query<&PieceColor, With<Piece>>) {

    for (&color, &position, mut moves, move_gen) in pieces {
        move_gen.generate(&mut commands, &mut moves, &board, position, color, piece_colors, true, true);
    }
}

pub fn generate_marker_captures<MoveGen: MoveGenerator + Component, Marker: PieceMarker + Component + Copy>(_event: On<GenerateMovesEvent>, mut commands: Commands, board: Single<&Board>,
    pieces: Query<(&PieceColor, &Position, &mut Moves, &CaptureMarker<MoveGen, Marker>), With<Piece>>, piece_colors: Query<&PieceColor, With<Piece>>,
    piece_positions: Query<&Position, With<Piece>>, markers: Query<(&Marker, &Position)>) {

    
    for (&color, &position, mut moves, CaptureMarker(move_gen, _)) in pieces {
        move_gen.generate_marker_captures(&mut commands, &mut moves, &board, position, color, piece_colors, MoveGen::get_marker_to_piece(markers, piece_positions));
    }
}

macro_rules! move_generator_plugin {
    ($plugin_name:ident,$move_generator:ty) => {
        pub struct $plugin_name;

        impl Plugin for $plugin_name {
            fn build(&self, app: &mut App) {
                app.add_observer(crate::chess::moves::move_generator::generate_moves::<$move_generator>)
                    .add_observer(crate::chess::moves::move_generator::generate_captures::<$move_generator>)
                    .add_observer(crate::chess::moves::move_generator::generate_moves_and_captures::<$move_generator>)
                    .add_observer(crate::chess::moves::move_generator::generate_marker_captures::<$move_generator, crate::chess::moves::pawn_moves::EnPassantMarker>);
            }
        }
    }
}

pub (crate) use move_generator_plugin;
