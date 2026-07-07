use std::ops::{Add, Index};

use bevy::{math::{USizeVec2, usizevec2}, platform::collections::HashSet, prelude::*};

pub const PIECE_SIZE: f32 = 48.0;
pub const BOARD_LENGTH: USizeVec2 = usizevec2(8, 8);
pub const BOARD_SIZE: Vec2 = vec2(BOARD_LENGTH.x as f32 * PIECE_SIZE, BOARD_LENGTH.y as f32 * PIECE_SIZE);

pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_board)
            .add_systems(Update, sync_transform_with_location);
    }
}

#[derive(Component)]
struct Board {
    pieces: [[Entity; BOARD_LENGTH.x]; BOARD_LENGTH.y],
    empty_piece: Entity,
}

impl Board {
    fn new(empty_piece: Entity) -> Self {
        Self {
            pieces: [[empty_piece; BOARD_LENGTH.x]; BOARD_LENGTH.y],
            empty_piece: empty_piece,
        }
    }
}

impl Index<Position> for Board {
    type Output = Entity;

    fn index(&self, position: Position) -> &Self::Output {
        &self.pieces[BOARD_LENGTH.x - 1 - position.rank as usize][position.file as usize]
    }
}

#[derive(Component)]
struct Piece;

#[derive(Component, Clone, Copy)]
#[require(Transform)]
struct Position {
    rank: isize,
    file: isize,
}

impl Position {
    fn new(rank: isize, file: isize) -> Self {
        Self {
            rank,
            file,
        }
    }

    fn to_translation(&self) -> Vec2 {
        let mut vec = Into::<Vec2>::into(*self);
        vec.x -= 3.5;
        vec.y = vec.y - 3.5;
        vec * PIECE_SIZE
    }
}

impl Into<Vec2> for Position {
    fn into(self) -> Vec2 {
        vec2(self.file as f32, self.rank as f32)
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, other: Direction) -> Self::Output {
        Self::new(self.rank + other.drank, self.file + other.dfile)
    }
}

struct Direction {
    drank: isize,
    dfile: isize,
}

impl Direction {
    const NORTH: Self = Self::new(0, 1);
    const SOUTH: Self = Self::new(0, -1);
    const EAST: Self = Self::new(1, 0);
    const WEST: Self = Self::new(-1, 0);
    const NORTH_EAST: Self = Self::new(1, 1);
    const NORTH_WEST: Self = Self::new(-1, 1);
    const SOUTH_EAST: Self = Self::new(1, -1);
    const SOUTH_WEST: Self = Self::new(-1, -1);

    const fn new(drank: isize, dfile: isize) -> Self {
        Self {
            drank,
            dfile,
        }
    }
}

impl Add for Direction {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.drank + other.drank, self.dfile + other.dfile)
    }
}

#[derive(Component)]
struct SlidingPiece(HashSet<Direction>);

fn piece(commands: &mut Commands, asset_server: &AssetServer, position: Position, texture_path: &'static str) -> Entity {
    let piece = commands.spawn((
        Piece,
        position,
        Sprite::from_image(asset_server.load(texture_path)),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));

    piece.id()
}

fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    let empty_piece = commands.spawn_empty().id();

    let mut pieces = vec![empty_piece];

    pieces.push(piece(&mut commands, &asset_server, Position::new(0, 0 as isize), "white/rook.png"));
    pieces.push(piece(&mut commands, &asset_server, Position::new(0, 1 as isize), "white/knight.png"));
    pieces.push(piece(&mut commands, &asset_server, Position::new(0, 2 as isize), "white/bishop.png"));
    pieces.push(piece(&mut commands, &asset_server, Position::new(0, 3 as isize), "white/queen.png"));
    pieces.push(piece(&mut commands, &asset_server, Position::new(0, 4 as isize), "white/king.png"));
    pieces.push(piece(&mut commands, &asset_server, Position::new(0, 5 as isize), "white/bishop.png"));
    pieces.push(piece(&mut commands, &asset_server, Position::new(0, 6 as isize), "white/knight.png"));
    pieces.push(piece(&mut commands, &asset_server, Position::new(0, 7 as isize), "white/rook.png"));

    pieces.push(piece(&mut commands, &asset_server, Position::new(7, 0 as isize), "black/rook.png"));
    pieces.push(piece(&mut commands, &asset_server, Position::new(7, 1 as isize), "black/knight.png"));
    pieces.push(piece(&mut commands, &asset_server, Position::new(7, 2 as isize), "black/bishop.png"));
    pieces.push(piece(&mut commands, &asset_server, Position::new(7, 3 as isize), "black/queen.png"));
    pieces.push(piece(&mut commands, &asset_server, Position::new(7, 4 as isize), "black/king.png"));
    pieces.push(piece(&mut commands, &asset_server, Position::new(7, 5 as isize), "black/bishop.png"));
    pieces.push(piece(&mut commands, &asset_server, Position::new(7, 6 as isize), "black/knight.png"));
    pieces.push(piece(&mut commands, &asset_server, Position::new(7, 7 as isize), "black/rook.png"));

    for file in 0..BOARD_LENGTH.y {
        pieces.push(piece(&mut commands, &asset_server, Position::new(1, file as isize), "white/pawn.png"));
        pieces.push(piece(&mut commands, &asset_server, Position::new(6, file as isize), "black/pawn.png"));
    }

    commands.spawn((
        Board::new(empty_piece),
        Sprite::from_image(asset_server.load("board.png")),
    )).add_children(&pieces);
}

fn sync_transform_with_location(mut entities: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in &mut entities {
        let vec = position.to_translation();

        transform.translation.x = vec.x;
        transform.translation.y = vec.y;
    }
}
