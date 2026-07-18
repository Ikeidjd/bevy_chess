use bevy::prelude::*;

use crate::{chess::{BOARD_LENGTH, board::{Board, BoardChanges}, markers::MarkerBoard, piece::{EmptyPiece, Piece, PieceColor}, position::Position, preset_pieces::{bishop, black_pawn, king, knight, queen, rook, white_pawn}}, layers};

pub (crate) fn spawn_piece(commands: &mut Commands, asset_server: &AssetServer, color: PieceColor, position: Position, texture_path: impl ToString, extra: impl Bundle) -> Entity {
    let mut piece = commands.spawn((
        Piece,
        color,
        position,
        Sprite::from_image(asset_server.load(texture_path.to_string())),
        Transform::from_xyz(0.0, 0.0, layers::PIECES),
    ));
    
    piece.insert(extra).id()
}

pub (crate) fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    let empty_piece = commands.spawn(EmptyPiece).id();

    let mut pieces = vec![empty_piece];

    pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::White, Position::new(0, 0 as isize), "white/rook.png", rook()));
    pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::White, Position::new(0, 1 as isize), "white/knight.png", knight()));
    pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::White, Position::new(0, 2 as isize), "white/bishop.png", bishop()));
    pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::White, Position::new(0, 3 as isize), "white/queen.png", queen()));
    pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::White, Position::new(0, 4 as isize), "white/king.png", king()));
    pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::White, Position::new(0, 5 as isize), "white/bishop.png", bishop()));
    pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::White, Position::new(0, 6 as isize), "white/knight.png", knight()));
    pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::White, Position::new(0, 7 as isize), "white/rook.png", rook()));

    pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::Black, Position::new(7, 0 as isize), "black/rook.png", rook()));
    pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::Black, Position::new(7, 1 as isize), "black/knight.png", knight()));
    pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::Black, Position::new(7, 2 as isize), "black/bishop.png", bishop()));
    pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::Black, Position::new(7, 3 as isize), "black/queen.png", queen()));
    pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::Black, Position::new(7, 4 as isize), "black/king.png", king()));
    pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::Black, Position::new(7, 5 as isize), "black/bishop.png", bishop()));
    pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::Black, Position::new(7, 6 as isize), "black/knight.png", knight()));
    pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::Black, Position::new(7, 7 as isize), "black/rook.png", rook()));

    for file in 0..BOARD_LENGTH.y {
        pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::White, Position::new(1, file as isize), "white/pawn.png", white_pawn()));
        pieces.push(spawn_piece(&mut commands, &asset_server, PieceColor::Black, Position::new(6, file as isize), "black/pawn.png", black_pawn()));
    }

    commands.spawn((
        Board::new(empty_piece),
        BoardChanges::default(),
        Sprite::from_image(asset_server.load("board.png")),
    )).add_children(&pieces);

    commands.spawn(MarkerBoard::default());
}

pub (crate) fn sync_pieces_with_board(pieces: Query<(Entity, &Position), With<Piece>>, mut board: Single<&mut Board>) {
    for (piece, &position) in &pieces {
        board[position] = piece;
    }
}
