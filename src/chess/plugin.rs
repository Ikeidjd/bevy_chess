use bevy::prelude::*;

use crate::chess::{ChessState, board::{check_board_clicked, on_board_pressed, on_board_released, restore_board}, markers::MarkerVisibilityPlugin, moves::{animation::{on_piece_animation_started, update_piece_animations}, castling_moves::generate_castling_moves, checks::check_illegal_moves, moves::{on_move_fully_ended, on_piece_moved}, pawn_moves::generate_pawn_moves, plugin::MovesPlugin, promotion::{attempt_promotion, check_promotion_option_clicked}}, piece::{on_piece_deselected, on_piece_selected, piece_follow_cursor, start_following_cursor, stop_following_cursor}, position::{on_sync_transform_with_position, sync_transform_with_position}, setup::{spawn_board, sync_pieces_with_board}};

pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MarkerVisibilityPlugin, MovesPlugin))
            .add_sub_state::<ChessState>()
            .add_systems(Startup, (spawn_board, sync_pieces_with_board).chain())
            .add_systems(Update, ((
                check_board_clicked.run_if(in_state(ChessState::Main)),
                (sync_transform_with_position, piece_follow_cursor).run_if(in_state(ChessState::Main)),
                update_piece_animations,
            ).chain(), check_promotion_option_clicked.run_if(in_state(ChessState::Promotion))))
            .add_observer(on_sync_transform_with_position)
            .add_observer(on_board_pressed)
            .add_observer(on_board_released)
            .add_observer(restore_board)
            .add_observer(on_piece_deselected)
            .add_observer(on_piece_selected)
            .add_observer(start_following_cursor)
            .add_observer(stop_following_cursor)
            .add_observer(generate_castling_moves)
            .add_observer(generate_pawn_moves)
            .add_observer(check_illegal_moves)
            .add_observer(attempt_promotion)
            .add_observer(on_move_fully_ended)
            .add_observer(on_piece_moved)
            .add_observer(on_piece_animation_started);
    }
}
