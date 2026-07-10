use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Direction {
    pub drank: isize,
    pub dfile: isize,
}

impl Direction {
    pub const NORTH: Self = Self::new(1, 0);
    pub const SOUTH: Self = Self::new(-1, 0);
    pub const EAST: Self = Self::new(0, 1);
    pub const WEST: Self = Self::new(0, -1);
    pub const NORTH_EAST: Self = Self::new(1, 1);
    pub const NORTH_WEST: Self = Self::new(1, -1);
    pub const SOUTH_EAST: Self = Self::new(-1, 1);
    pub const SOUTH_WEST: Self = Self::new(-1, -1);

    pub const ORTHOGONAL: [Self; 4] = [
        Direction::NORTH,
        Direction::SOUTH,
        Direction::EAST,
        Direction::WEST,
    ];

    pub const DIAGONAL: [Self; 4] = [
        Direction::NORTH_EAST,
        Direction::NORTH_WEST,
        Direction::SOUTH_EAST,
        Direction::SOUTH_WEST,
    ];

    pub const MONARCH: [Self; 8] = [
        Direction::NORTH,
        Direction::SOUTH,
        Direction::EAST,
        Direction::WEST,
        Direction::NORTH_EAST,
        Direction::NORTH_WEST,
        Direction::SOUTH_EAST,
        Direction::SOUTH_WEST,
    ];

    pub const KNIGHT: [Self; 8] = [
        Direction::new(2, 1),
        Direction::new(2, -1),
        Direction::new(-2, 1),
        Direction::new(-2, -1),
        Direction::new(1, 2),
        Direction::new(-1, 2),
        Direction::new(1, -2),
        Direction::new(-1, -2),
    ];

    pub const fn new(drank: isize, dfile: isize) -> Self {
        Self {
            drank,
            dfile,
        }
    }

    pub fn normalize(&self) -> Self {
        Self::new(self.drank.signum(), self.dfile.signum())
    }
}

impl Add for Direction {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.drank + other.drank, self.dfile + other.dfile)
    }
}

impl Sub for Direction {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.drank - other.drank, self.dfile - other.dfile)
    }
}

impl Mul<isize> for Direction {
    type Output = Self;

    fn mul(self, other: isize) -> Self::Output {
        Self::new(self.drank * other, self.dfile * other)
    }
}
