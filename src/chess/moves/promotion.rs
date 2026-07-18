use bevy::prelude::*;

use crate::{CursorWorldCoordinates, chess::{BOARD_LENGTH, ChessState, PIECE_SIZE, board::{Board, BoardChanges}, direction::Direction, moves::moves::MoveFullyEndedEvent, piece::{Piece, PieceColor}, position::{Position, SyncTransformWithPosition}, preset_pieces::{bishop, knight, queen, rook}, setup::spawn_piece}, layers};

#[derive(Component)]
pub (crate) struct PromotingPiece;

#[derive(Component)]
pub (crate) struct PromotionOption(pub (crate) Entity);

#[derive(Component)]
pub (crate) struct PromotionCancel;

#[derive(Event)]
pub (crate) struct AttemptPromotionEvent(pub (crate) Entity);

pub (crate) fn attempt_promotion(event: On<AttemptPromotionEvent>, mut commands: Commands, mut next_state: ResMut<NextState<ChessState>>, asset_server: Res<AssetServer>,
    pieces: Query<(&PieceColor, &Position), (With<Piece>, With<PromotingPiece>)>) {

    let (&piece_color, &position) = match pieces.get(event.0) {
        Ok(piece) => piece,
        Err(_) => {
            commands.trigger(MoveFullyEndedEvent);
            return;
        }
    };

    let dir = if position.rank == 0 {
        Direction::NORTH
    } else if position.rank as usize == BOARD_LENGTH.x - 1 {
        Direction::SOUTH
    } else {
        commands.trigger(MoveFullyEndedEvent);
        return;
    };

    let color = match piece_color {
        PieceColor::White => "white",
        PieceColor::Black => "black",
    };

    let mut pieces = vec![
        spawn_piece(&mut commands, &asset_server, piece_color, position, format!("{color}/queen.png"), queen()),
        spawn_piece(&mut commands, &asset_server, piece_color, position + dir, format!("{color}/rook.png"), rook()),
        spawn_piece(&mut commands, &asset_server, piece_color, position + dir * 2, format!("{color}/bishop.png"), bishop()),
        spawn_piece(&mut commands, &asset_server, piece_color, position + dir * 3, format!("{color}/knight.png"), knight()),
    ];

    for &piece in &pieces {
        commands.entity(piece).insert(PromotionOption(event.0));
    }

    pieces.push(commands.spawn((
        PromotionCancel,
        position + dir * 4,
        Sprite::from_image(asset_server.load("promotion_cancel.png")),
    )).id());

    commands.trigger(SyncTransformWithPosition);

    let translation = position.to_translation() + vec2(dir.dfile as f32, dir.drank as f32) * PIECE_SIZE * (pieces.len() - 1) as f32 * 0.5;

    let background = commands.spawn((
        Sprite::from_color(Color::WHITE, (vec2(dir.dfile.abs() as f32, dir.drank.abs() as f32) * (pieces.len() - 1) as f32 + vec2(1.0, 1.0)) * PIECE_SIZE),
        Transform::from_xyz(translation.x, translation.y, 0.0),
    )).id();

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, layers::PROMOTION_MENU),
        Visibility::Visible,
        DespawnOnExit(ChessState::Promotion),
    )).add_child(background).add_children(&pieces);

    next_state.set(ChessState::Promotion);
}

pub (crate) fn check_promotion_option_clicked(mut commands: Commands, mut next_state: ResMut<NextState<ChessState>>, cursor: Res<CursorWorldCoordinates>, input: Res<ButtonInput<MouseButton>>,
    mut board: Single<(Entity, &mut Board, &mut BoardChanges)>, pieces: Query<(Entity, &Position, &Transform), (With<Piece>, With<PromotingPiece>)>,
    promotion_options: Query<(Entity, &Position, &PromotionOption)>, promotion_cancel_position: Single<&Position, With<PromotionCancel>>) {

    if !input.just_released(MouseButton::Left) {
        return;
    }

    let cursor = Position::from_translation(cursor.0);

    let (board_entity, ref mut board, ref mut board_changes) = *board;

    if cursor == **promotion_cancel_position {
        board.restore_changes(&mut commands, board_changes);
        next_state.set(ChessState::Main);
        return;
    }

    for (promotion_option_entity, &promotion_option_position, promotion_option) in promotion_options {
        if cursor == promotion_option_position {
            let (piece, &position, &transform) = pieces.get(promotion_option.0).unwrap();

            commands.entity(piece).despawn();
            commands.entity(promotion_option_entity).remove::<(PromotionOption, ChildOf)>().insert((position, transform));

            let piece = promotion_option_entity;

            commands.entity(board_entity).add_child(piece);
            board[position] = piece;

            next_state.set(ChessState::Main);

            commands.trigger(MoveFullyEndedEvent);
            return;
        }
    }
}
