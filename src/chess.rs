use std::ops::{Add, Index, IndexMut};

use bevy::{math::{USizeVec2, usizevec2}, platform::collections::HashSet, prelude::*};

use crate::CursorWorldCoordinates;

pub const PIECE_SIZE: f32 = 48.0;
pub const BOARD_LENGTH: USizeVec2 = usizevec2(8, 8);
pub const BOARD_SIZE: Vec2 = vec2(BOARD_LENGTH.x as f32 * PIECE_SIZE, BOARD_LENGTH.y as f32 * PIECE_SIZE);

pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_board, sync_pieces_with_board).chain())
            .add_systems(Update, (check_board_clicked, sync_transform_with_position, piece_follow_cursor))
            .add_observer(on_board_clicked)
            .add_observer(on_piece_selected);
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

    fn in_bounds(&self, position: Position) -> bool {
        position.rank >= 0 && position.rank < BOARD_LENGTH.x as isize && position.file >= 0 && position.file < BOARD_LENGTH.y as isize
    }
}

impl Index<Position> for Board {
    type Output = Entity;

    fn index(&self, position: Position) -> &Self::Output {
        &self.pieces[position.rank as usize][position.file as usize]
    }
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        &mut self.pieces[position.rank as usize][position.file as usize]
    }
}

#[derive(Component)]
struct Piece;

#[derive(Component)]
struct SelectedPiece { yellow_square: Entity }

#[derive(Component)]
struct EmptyPiece;

// If I don't put the starting transform off-screen, it appears in the center of the board for one frame, which looks very bad
#[derive(Component)]
#[require(Sprite::from_color(Color::srgba(1.0, 1.0, 0.0, 0.5), vec2(PIECE_SIZE, PIECE_SIZE)), Transform::from_xyz(-1000.0, -1000.0, 0.5))]
struct YellowSquare;

#[derive(Component)]
struct PieceFollowsCursor;

#[derive(Component, Debug, Clone, Copy)]
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

    fn from_translation(vec: Vec2) -> Self {
        // The reason this is 4.0 instead of 3.5 is that this one is not concerned with the center, but with the corner
        let position = vec / PIECE_SIZE + vec2(4.0, 4.0);

        // Flooring is needed because, otherwise, the squares right past the bottom-left corner turn into (0, 0) instead of (-1, 0), (0, -1) and (-1, -1) due to truncation
        Self::new(position.y.floor() as isize, position.x.floor() as isize)
    }

    fn to_translation(&self) -> Vec2 {
        // This returns the center of the piece's transform.translation
        (vec2(self.file as f32, self.rank as f32) - vec2(3.5, 3.5)) * PIECE_SIZE
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

#[derive(Event)]
struct BoardClickedEvent(Position);

#[derive(Event)]
struct PieceSelectedEvent(Entity);

fn check_board_clicked(mut commands: Commands, input: Res<ButtonInput<MouseButton>>, cursor: Res<CursorWorldCoordinates>, piece_follows_cursor: Query<Entity, With<PieceFollowsCursor>>) {
    if input.just_pressed(MouseButton::Left) {
        let cursor = Position::from_translation(cursor.0);
        println!("{:?}", cursor);
        commands.trigger(BoardClickedEvent(cursor));
    } else if input.just_released(MouseButton::Left) && let Ok(piece) = piece_follows_cursor.single() {
        commands.entity(piece).remove::<PieceFollowsCursor>();
    }
}

fn on_board_clicked(event: On<BoardClickedEvent>, mut commands: Commands, board: Single<&Board>,
    selected_piece: Query<(Entity, &Position, &SelectedPiece), (With<Piece>, With<SelectedPiece>)>, empty_piece: Single<Entity, With<EmptyPiece>>) {

    if !board.in_bounds(event.0) {
        return;
    }

    let next_selected = board[event.0];

    if let Ok((selected_piece_entity, selected_piece_position, selected_piece)) = selected_piece.single() {
        // The yellow square disappears for one frame when reselecting, so we simply don't reselect, but we do make the piece follow the cursor again
        if next_selected == selected_piece_entity {
            commands.entity(selected_piece_entity).insert(PieceFollowsCursor);
            return;
        }

        if next_selected == *empty_piece {
            // TODO: add the ability to move
        }

        commands.entity(selected_piece.yellow_square).despawn();
        commands.entity(selected_piece_entity).remove::<(SelectedPiece, PieceFollowsCursor)>();
    }

    commands.trigger(PieceSelectedEvent(next_selected));
}

fn on_piece_selected(event: On<PieceSelectedEvent>, mut commands: Commands, pieces: Query<(Entity, &Position), With<Piece>>) {
    let (piece, &position) = match pieces.get(event.0) {
        Ok(piece) => piece,
        Err(_) => return,
    };

    let yellow_square = commands.spawn((
        YellowSquare,
        position,
    )).id();

    commands.entity(piece).insert((SelectedPiece { yellow_square }, PieceFollowsCursor));
}

fn sync_transform_with_position(mut entities: Query<(&Position, &mut Transform), Without<PieceFollowsCursor>>) {
    for (position, mut transform) in &mut entities {
        let vec = position.to_translation();

        transform.translation.x = vec.x;
        transform.translation.y = vec.y;
    }
}

fn piece_follow_cursor(cursor: Res<CursorWorldCoordinates>, mut piece: Single<&mut Transform, With<PieceFollowsCursor>>) {
    piece.translation.x = cursor.0.x;
    piece.translation.y = cursor.0.y;
}

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
    let empty_piece = commands.spawn(EmptyPiece).id();

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

fn sync_pieces_with_board(pieces: Query<(Entity, &Position), With<Piece>>, mut board: Single<&mut Board>) {
    for (piece, &position) in &pieces {
        board[position] = piece;
    }
}
