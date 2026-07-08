use std::ops::{Add, AddAssign, Index, IndexMut};

use bevy::{ecs::reflect::ReflectCommandExt, math::{USizeVec2, usizevec2}, platform::collections::HashSet, prelude::*};

use crate::CursorWorldCoordinates;

pub const PIECE_SIZE: f32 = 48.0;
pub const BOARD_LENGTH: USizeVec2 = usizevec2(8, 8);
pub const BOARD_SIZE: Vec2 = vec2(BOARD_LENGTH.x as f32 * PIECE_SIZE, BOARD_LENGTH.y as f32 * PIECE_SIZE);

pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_board, sync_pieces_with_board).chain())
            .add_systems(Update, (check_board_clicked, (sync_transform_with_position, piece_follow_cursor)).chain())
            .add_observer(on_board_pressed)
            .add_observer(on_board_released)
            .add_observer(on_piece_deselected)
            .add_observer(on_piece_selected)
            .add_observer(on_generate_moves)
            .add_observer(on_piece_moved);
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

    fn is_in_bounds(&self, position: Position) -> bool {
        position.rank >= 0 && position.rank < BOARD_LENGTH.x as isize && position.file >= 0 && position.file < BOARD_LENGTH.y as isize
    }

    fn is_empty(&self, position: Position) -> bool {
        self[position] == self.empty_piece
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

#[derive(Component, PartialEq, Eq, Clone, Copy)]
enum PieceColor {
    White,
    Black,
}

#[derive(Component)]
struct SelectedPiece { yellow_square: Entity }

#[derive(Component)]
struct EmptyPiece;

#[derive(Component)]
#[require(Sprite::from_color(Color::srgba(1.0, 1.0, 0.0, 0.5), vec2(PIECE_SIZE, PIECE_SIZE)))]
struct YellowSquare;

#[derive(Component)]
#[require(Sprite::from_color(Color::BLACK.with_alpha(0.75), vec2(PIECE_SIZE, PIECE_SIZE)))]
struct BlackCircle;

#[derive(Component)]
struct PieceFollowsCursor;

#[derive(Component, Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, other: Direction) {
        self.rank += other.drank;
        self.file += other.dfile;
    }
}

#[derive(Reflect, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Component, Default)]
struct Moves {
    positions: HashSet<Position>,
    black_circles: Vec<Entity>,
}

impl Moves {
    fn insert(&mut self, commands: &mut Commands, position: Position) {
        if self.positions.insert(position) {
            self.black_circles.push(commands.spawn((
                BlackCircle,
                position,
            )).id());
        }
    }
}

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
struct SlidingMoveGenerator(HashSet<Direction>);

#[derive(Event)]
struct BoardPressedEvent(Position);

#[derive(Event)]
struct BoardReleasedEvent(Position);

#[derive(Event)]
struct PieceSelectedEvent(Entity);

#[derive(Event)]
struct PieceDeselectedEvent;

#[derive(Event)]
struct GenerateMovesEvent;

#[derive(Event)]
struct PieceMovedEvent(Position, Position);

fn check_board_clicked(mut commands: Commands, input: Res<ButtonInput<MouseButton>>, cursor: Res<CursorWorldCoordinates>) {
    let cursor = Position::from_translation(cursor.0);

    if input.just_pressed(MouseButton::Left) {
        commands.trigger(BoardPressedEvent(cursor));
    } else if input.just_released(MouseButton::Left) {
        commands.trigger(BoardReleasedEvent(cursor));
    }
}

fn on_board_pressed(event: On<BoardPressedEvent>, mut commands: Commands, board: Single<&Board>, selected_piece: Query<&Moves, With<Piece>>) {
    if let Ok(moves) = selected_piece.single() && moves.positions.contains(&event.0) {
        return;
    }

    if board.is_in_bounds(event.0) {
        commands.trigger(PieceDeselectedEvent);
        commands.trigger(PieceSelectedEvent(board[event.0]));
    }
}

fn on_board_released(event: On<BoardReleasedEvent>, mut commands: Commands, board: Single<&Board>, selected_piece: Query<(Entity, &Position, &Moves), With<Piece>>) {
    if board.is_in_bounds(event.0) && let Ok((selected_piece_entity, &selected_piece_position, moves)) = selected_piece.single() {
        match moves.positions.contains(&event.0) {
            true => {
                commands.trigger(PieceMovedEvent(selected_piece_position, event.0));
            }
            false => {
                commands.entity(selected_piece_entity).remove::<PieceFollowsCursor>();
            }
        }
    }
}

fn on_piece_deselected(_event: On<PieceDeselectedEvent>, mut commands: Commands, selected_piece: Single<(Entity, &SelectedPiece, &Moves), With<Piece>>) {
    let (selected_piece_entity, selected_piece, moves) = *selected_piece;

    commands.entity(selected_piece.yellow_square).despawn();

    for &black_circle in &moves.black_circles {
        commands.entity(black_circle).despawn();
    }

    commands.entity(selected_piece_entity).remove::<(SelectedPiece, PieceFollowsCursor, Moves)>();
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

    commands.entity(piece).insert((
        SelectedPiece { yellow_square },
        PieceFollowsCursor,
        Moves::default(),
    ));

    commands.trigger(GenerateMovesEvent);
}

fn on_generate_moves(_event: On<GenerateMovesEvent>, mut commands: Commands,
    mut piece: Single<(&PieceColor, &Position, &mut Moves, &SlidingMoveGenerator), (With<Piece>, With<SelectedPiece>)>, board: Single<&Board>, pieces: Query<&PieceColor, With<Piece>>) {

    let (color, &position, ref mut moves, move_gen) = *piece;

    for &dir in &move_gen.0 {
        let mut pos = position + dir;

        while board.is_in_bounds(pos) && board.is_empty(pos) {
            moves.insert(&mut commands, pos);
            pos += dir;
        }

        if !board.is_in_bounds(pos) {
            continue;
        }

        let target_color = match pieces.get(board[pos]) {
            Ok(color) => color,
            Err(_) => continue, // Will happen when I implement duck chess, since the duck has no color
        };

        if color != target_color {
            moves.insert(&mut commands, pos);
        }
    }
}

fn on_piece_moved(event: On<PieceMovedEvent>, mut commands: Commands, mut board: Single<&mut Board>, mut pieces: Query<(Entity, &mut Position), With<Piece>>) {
    commands.trigger(PieceDeselectedEvent);

    let PieceMovedEvent(from, to) = *event;
    let (piece, mut piece_position) = pieces.get_mut(board[from]).unwrap();

    if !board.is_empty(to) {
        commands.entity(board[to]).despawn();
    }

    board[from] = board.empty_piece;
    board[to] = piece;

    *piece_position = to;
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

fn piece(commands: &mut Commands, asset_server: &AssetServer, color: PieceColor, position: Position, texture_path: &'static str, extra: Box<dyn PartialReflect>) -> Entity {
    let mut piece = commands.spawn((
        Piece,
        color,
        position,
        Sprite::from_image(asset_server.load(texture_path)),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
    
    piece.insert_reflect(extra).id()
}

fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    let empty_piece = commands.spawn(EmptyPiece).id();

    let mut pieces = vec![empty_piece];

    let orthogonal = SlidingMoveGenerator(HashSet::from([
        Direction::NORTH,
        Direction::SOUTH,
        Direction::EAST,
        Direction::WEST,
    ]));

    let diagonal = SlidingMoveGenerator(HashSet::from([
        Direction::NORTH_EAST,
        Direction::NORTH_WEST,
        Direction::SOUTH_EAST,
        Direction::SOUTH_WEST,
    ]));

    let queen = SlidingMoveGenerator(HashSet::from([
        Direction::NORTH,
        Direction::SOUTH,
        Direction::EAST,
        Direction::WEST,
        Direction::NORTH_EAST,
        Direction::NORTH_WEST,
        Direction::SOUTH_EAST,
        Direction::SOUTH_WEST,
    ]));

    pieces.push(piece(&mut commands, &asset_server, PieceColor::White, Position::new(0, 0 as isize), "white/rook.png", Box::new(orthogonal.clone())));
    pieces.push(piece(&mut commands, &asset_server, PieceColor::White, Position::new(0, 2 as isize), "white/bishop.png", Box::new(diagonal.clone())));
    pieces.push(piece(&mut commands, &asset_server, PieceColor::White, Position::new(0, 3 as isize), "white/queen.png", Box::new(queen.clone())));
    pieces.push(piece(&mut commands, &asset_server, PieceColor::White, Position::new(0, 5 as isize), "white/bishop.png", Box::new(diagonal.clone())));
    pieces.push(piece(&mut commands, &asset_server, PieceColor::White, Position::new(0, 7 as isize), "white/rook.png", Box::new(orthogonal.clone())));

    pieces.push(piece(&mut commands, &asset_server, PieceColor::Black, Position::new(7, 0 as isize), "black/rook.png", Box::new(orthogonal.clone())));
    pieces.push(piece(&mut commands, &asset_server, PieceColor::Black, Position::new(7, 2 as isize), "black/bishop.png", Box::new(diagonal.clone())));
    pieces.push(piece(&mut commands, &asset_server, PieceColor::Black, Position::new(7, 3 as isize), "black/queen.png", Box::new(queen.clone())));
    pieces.push(piece(&mut commands, &asset_server, PieceColor::Black, Position::new(7, 5 as isize), "black/bishop.png", Box::new(diagonal.clone())));
    pieces.push(piece(&mut commands, &asset_server, PieceColor::Black, Position::new(7, 7 as isize), "black/rook.png", Box::new(orthogonal.clone())));

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
