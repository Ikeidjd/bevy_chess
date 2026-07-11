use bevy::{math::{USizeVec2, usizevec2}, prelude::*};

use crate::{MainState, chess::{board::{check_board_clicked, on_board_pressed, on_board_released}, moves::{castling_moves::generate_castling_moves, on_move_fully_ended, on_piece_animation_started, on_piece_moved, pawn_moves::generate_pawn_moves, promotion::{attempt_promotion, check_promotion_option_clicked}, single_moves::{generate_single_captures, generate_single_moves, generate_single_moves_and_captures}, sliding_moves::{generate_sliding_captures, generate_sliding_moves, generate_sliding_moves_and_captures}, update_piece_animations}, piece::{on_piece_deselected, on_piece_selected, piece_follow_cursor, start_following_cursor, stop_following_cursor}, position::{on_sync_transform_with_position, sync_transform_with_position}, setup::{spawn_board, sync_pieces_with_board}}};

mod position;
mod direction;
mod preset_pieces;
mod setup;
mod board;
mod piece;
mod moves;

pub const PIECE_SIZE: f32 = 48.0;
pub const BOARD_LENGTH: USizeVec2 = usizevec2(8, 8);

pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<ChessState>()
            .add_systems(Startup, (spawn_board, sync_pieces_with_board).chain())
            .add_systems(Update, ((
                check_board_clicked.run_if(in_state(ChessState::Main)),
                (sync_transform_with_position, piece_follow_cursor).run_if(in_state(ChessState::Main)),
                update_piece_animations,
            ).chain(), check_promotion_option_clicked.run_if(in_state(ChessState::Promotion))))
            .add_observer(on_sync_transform_with_position)
            .add_observer(on_board_pressed)
            .add_observer(on_board_released)
            .add_observer(on_piece_deselected)
            .add_observer(on_piece_selected)
            .add_observer(start_following_cursor)
            .add_observer(stop_following_cursor)
            .add_observer(generate_single_moves)
            .add_observer(generate_single_captures)
            .add_observer(generate_single_moves_and_captures)
            .add_observer(generate_sliding_moves)
            .add_observer(generate_sliding_captures)
            .add_observer(generate_sliding_moves_and_captures)
            .add_observer(generate_castling_moves)
            .add_observer(generate_pawn_moves)
            .add_observer(attempt_promotion)
            .add_observer(on_move_fully_ended)
            .add_observer(on_piece_moved)
            .add_observer(on_piece_animation_started);
    }
}

#[derive(SubStates, Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
#[source(MainState = MainState::Chess)]
enum ChessState {
    #[default]
    Main,
    PieceAnimation,
    Promotion,
}
