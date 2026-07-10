use std::ops::{Add, AddAssign, Sub};

use bevy::prelude::*;

use crate::chess::{PIECE_SIZE, direction::Direction, piece::PieceFollowsCursor};

#[derive(Component, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[require(Transform)]
pub struct Position {
    pub rank: isize,
    pub file: isize,
}

impl Position {
    pub fn new(rank: isize, file: isize) -> Self {
        Self {
            rank,
            file,
        }
    }

    pub fn from_translation(vec: Vec2) -> Self {
        // The reason this is 4.0 instead of 3.5 is that this one is not concerned with the center, but with the corner
        let position = vec / PIECE_SIZE + vec2(4.0, 4.0);

        // Flooring is needed because, otherwise, the squares right past the bottom-left corner turn into (0, 0) instead of (-1, 0), (0, -1) and (-1, -1) due to truncation
        Self::new(position.y.floor() as isize, position.x.floor() as isize)
    }

    pub fn to_translation(&self) -> Vec2 {
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

impl Sub for Position {
    type Output = Direction;

    fn sub(self, other: Self) -> Self::Output {
        Direction::new(self.rank - other.rank, self.file - other.file)
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, other: Direction) {
        self.rank += other.drank;
        self.file += other.dfile;
    }
}

#[derive(Event)]
pub struct SyncTransformWithPosition;

pub fn sync_transform_with_position(mut commands: Commands) {
    commands.trigger(SyncTransformWithPosition);
}

pub fn on_sync_transform_with_position(_event: On<SyncTransformWithPosition>, entities: Query<(&Position, &mut Transform), Without<PieceFollowsCursor>>) {
    for (position, mut transform) in entities {
        let vec = position.to_translation();

        transform.translation.x = vec.x;
        transform.translation.y = vec.y;
    }
}
