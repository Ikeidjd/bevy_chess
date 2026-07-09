use bevy::{platform::collections::HashSet, prelude::*};

use crate::chess::{board::Board, direction::Direction, moves::{castling_moves::{CastleBottom, CastleTop}, single_moves::SingleMoveGenerator, sliding_moves::SlidingMoveGenerator}, piece::{EmptyPiece, Piece, PieceColor}, position::Position};

pub fn piece(commands: &mut Commands, asset_server: &AssetServer, color: PieceColor, position: Position, texture_path: &'static str, extra: impl Bundle) -> Entity {
    let mut piece = commands.spawn((
        Piece,
        color,
        position,
        Sprite::from_image(asset_server.load(texture_path)),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
    
    piece.insert(extra).id()
}

pub fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    let empty_piece = commands.spawn(EmptyPiece).id();

    let mut pieces = vec![empty_piece];

    let orthogonal = HashSet::from([
        Direction::NORTH,
        Direction::SOUTH,
        Direction::EAST,
        Direction::WEST,
    ]);

    let diagonal = HashSet::from([
        Direction::NORTH_EAST,
        Direction::NORTH_WEST,
        Direction::SOUTH_EAST,
        Direction::SOUTH_WEST,
    ]);

    let monarch: HashSet<_> = orthogonal.union(&diagonal).map(|&direction| direction).collect();

    let mut knight = HashSet::new();

    for &dir in &orthogonal {
        knight.insert(dir * 2 + Direction::new(dir.dfile, dir.drank));
        knight.insert(dir * 2 - Direction::new(dir.dfile, dir.drank));
    }

    pieces.push(piece(&mut commands, &asset_server, PieceColor::White, Position::new(0, 0 as isize), "white/rook.png", (
        SlidingMoveGenerator(orthogonal.clone()),
        CastleBottom,
    )));

    pieces.push(piece(&mut commands, &asset_server, PieceColor::White, Position::new(0, 7 as isize), "white/rook.png", (
        SlidingMoveGenerator(orthogonal.clone()),
        CastleBottom,
    )));

    pieces.push(piece(&mut commands, &asset_server, PieceColor::Black, Position::new(7, 0 as isize), "black/rook.png", (
        SlidingMoveGenerator(orthogonal.clone()),
        CastleBottom,
    )));
    
    pieces.push(piece(&mut commands, &asset_server, PieceColor::Black, Position::new(7, 7 as isize), "black/rook.png", (
        SlidingMoveGenerator(orthogonal.clone()),
        CastleBottom,
    )));

    pieces.push(piece(&mut commands, &asset_server, PieceColor::White, Position::new(0, 4 as isize), "white/king.png", (
        SingleMoveGenerator(monarch.clone()),
        CastleTop,
    )));

    pieces.push(piece(&mut commands, &asset_server, PieceColor::Black, Position::new(7, 4 as isize), "black/king.png", (
        SingleMoveGenerator(monarch.clone()),
        CastleTop,
    )));

    commands.spawn((
        Board::new(empty_piece),
        Sprite::from_image(asset_server.load("board.png")),
    )).add_children(&pieces);
}

pub fn sync_pieces_with_board(pieces: Query<(Entity, &Position), With<Piece>>, mut board: Single<&mut Board>) {
    for (piece, &position) in &pieces {
        board[position] = piece;
    }
}
