use std::marker::PhantomData;

use bevy::{ecs::query::QueryFilter, platform::collections::HashMap, prelude::*};

use crate::chess::{board::Board, markers::{MarkerBoard, PieceMarker}, moves::moves::{GenerateMovesEvent, Moves}, piece::{Piece, PieceColor}, position::Position};

pub (crate) trait MoveGenerator {
    fn generate<F: QueryFilter>(&self, commands: &mut Commands, moves: &mut Moves, board: &Board, position: Position, color: PieceColor, piece_colors: Query<&PieceColor, F>,
        allow_moves: bool, allow_captures: bool);

    fn generate_marker_captures<F: QueryFilter>(&self, commands: &mut Commands, moves: &mut Moves, board: &Board, position: Position, color: PieceColor,
        piece_colors: Query<&PieceColor, F>, marker_to_piece: HashMap<Position, Position>);

    fn get_marker_to_piece<Marker: PieceMarker + Component + Copy, F: QueryFilter>(marker_board: &MarkerBoard, markers: Query<(&Marker, &Position), F>,
        piece_positions: Query<&Position, With<Piece>>) -> HashMap<Position, Position> {

        marker_board.current.iter().filter_map(|&marker_entity| match markers.get(marker_entity) {
            Ok((&marker, &position)) => Some((position, piece_positions.get(marker.get_entity()).unwrap().clone())),
            Err(_) => None,
        }).collect()
    }
}

#[derive(Component)]
pub (crate) struct MoveOnly<MoveGen: MoveGenerator + Component>(pub (crate) MoveGen);

#[derive(Component)]
pub (crate) struct CaptureOnly<MoveGen: MoveGenerator + Component>(pub (crate) MoveGen);

#[derive(Component)]
pub (crate) struct CapturesMarkers<MoveGen: MoveGenerator + Component, Marker: PieceMarker + Component>(pub (crate) MoveGen, pub (crate) PhantomData<Marker>);

#[derive(Component, Default)]
pub (crate) struct CapturesMarkersAlways<Marker: PieceMarker + Component>(pub (crate) PhantomData<Marker>);

pub (crate) fn generate_moves<MoveGen: MoveGenerator + Component>(_event: On<GenerateMovesEvent>, mut commands: Commands, board: Single<&Board>,
    pieces: Query<(&PieceColor, &Position, &mut Moves, &MoveOnly<MoveGen>), With<Piece>>, piece_colors: Query<&PieceColor, With<Piece>>) {

    for (&color, &position, mut moves, MoveOnly(move_gen)) in pieces {
        move_gen.generate(&mut commands, &mut moves, &board, position, color, piece_colors, true, false);
    }
}

pub (crate) fn generate_captures<MoveGen: MoveGenerator + Component>(_event: On<GenerateMovesEvent>, mut commands: Commands, board: Single<&Board>,
    pieces: Query<(&PieceColor, &Position, &mut Moves, &CaptureOnly<MoveGen>), With<Piece>>, piece_colors: Query<&PieceColor, With<Piece>>) {

    for (&color, &position, mut moves, CaptureOnly(move_gen)) in pieces {
        move_gen.generate(&mut commands, &mut moves, &board, position, color, piece_colors, false, true);
    }
}

pub (crate) fn generate_moves_and_captures<MoveGen: MoveGenerator + Component>(_event: On<GenerateMovesEvent>, mut commands: Commands, board: Single<&Board>,
    pieces: Query<(&PieceColor, &Position, &mut Moves, &MoveGen), With<Piece>>, piece_colors: Query<&PieceColor, With<Piece>>) {

    for (&color, &position, mut moves, move_gen) in pieces {
        move_gen.generate(&mut commands, &mut moves, &board, position, color, piece_colors, true, true);
    }
}

pub (crate) fn generate_marker_captures<MoveGen: MoveGenerator + Component, Marker: PieceMarker + Component + Copy>(_event: On<GenerateMovesEvent>, mut commands: Commands,
    board: Single<&Board>, marker_board: Single<&MarkerBoard>,
    pieces: Query<(&PieceColor, &Position, &mut Moves, &CapturesMarkers<MoveGen, Marker>), With<Piece>>, piece_colors: Query<&PieceColor, With<Piece>>,
    piece_positions: Query<&Position, With<Piece>>, markers: Query<(&Marker, &Position)>) {

    for (&color, &position, mut moves, CapturesMarkers(move_gen, _)) in pieces {
        move_gen.generate_marker_captures(&mut commands, &mut moves, &board, position, color, piece_colors, MoveGen::get_marker_to_piece(&marker_board, markers, piece_positions));
    }
}

pub (crate) fn generate_marker_captures_for_capture_only<MoveGen: MoveGenerator + Component, Marker: PieceMarker + Component + Copy>(_event: On<GenerateMovesEvent>, mut commands: Commands,
    board: Single<&Board>, marker_board: Single<&MarkerBoard>,
    pieces: Query<(&PieceColor, &Position, &mut Moves, &CaptureOnly<MoveGen>), (With<Piece>, With<CapturesMarkersAlways<Marker>>)>, piece_colors: Query<&PieceColor, With<Piece>>,
    piece_positions: Query<&Position, With<Piece>>, markers: Query<(&Marker, &Position)>) {

    for (&color, &position, mut moves, CaptureOnly(move_gen)) in pieces {
        move_gen.generate_marker_captures(&mut commands, &mut moves, &board, position, color, piece_colors, MoveGen::get_marker_to_piece(&marker_board, markers, piece_positions));
    }
}

pub (crate) fn generate_marker_captures_for_move_gen<MoveGen: MoveGenerator + Component, Marker: PieceMarker + Component + Copy>(_event: On<GenerateMovesEvent>, mut commands: Commands,
    board: Single<&Board>, marker_board: Single<&MarkerBoard>,
    pieces: Query<(&PieceColor, &Position, &mut Moves, &MoveGen), (With<Piece>, With<CapturesMarkersAlways<Marker>>)>, piece_colors: Query<&PieceColor, With<Piece>>,
    piece_positions: Query<&Position, With<Piece>>, markers: Query<(&Marker, &Position)>) {

    for (&color, &position, mut moves, move_gen) in pieces {
        move_gen.generate_marker_captures(&mut commands, &mut moves, &board, position, color, piece_colors, MoveGen::get_marker_to_piece(&marker_board, markers, piece_positions));
    }
}

macro_rules! register_marker {
    ($app:ident,$move_generator:ty,$marker:ty) => {
        $app.add_observer(crate::chess::moves::move_generator::generate_marker_captures::<$move_generator, $marker>)
            .add_observer(crate::chess::moves::move_generator::generate_marker_captures_for_capture_only::<$move_generator, $marker>)
            .add_observer(crate::chess::moves::move_generator::generate_marker_captures_for_move_gen::<$move_generator, $marker>)
    }
}

macro_rules! move_generator_plugin {
    ($plugin_name:ident,$move_generator:ty) => {
        pub (crate) struct $plugin_name;

        impl Plugin for $plugin_name {
            fn build(&self, app: &mut App) {
                app.add_observer(crate::chess::moves::move_generator::generate_moves::<$move_generator>)
                    .add_observer(crate::chess::moves::move_generator::generate_captures::<$move_generator>)
                    .add_observer(crate::chess::moves::move_generator::generate_moves_and_captures::<$move_generator>);
                crate::chess::moves::move_generator::register_marker!(app, $move_generator, crate::chess::markers::EnPassantMarker);
                crate::chess::moves::move_generator::register_marker!(app, $move_generator, crate::chess::markers::CastleMarker);
            }
        }
    }
}

pub (crate) (crate) use register_marker;
pub (crate) (crate) use move_generator_plugin;
