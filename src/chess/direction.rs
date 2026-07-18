use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
pub (crate) struct Direction {
    pub (crate) drank: isize,
    pub (crate) dfile: isize,
}

impl Direction {
    pub (crate) const NORTH: Self = Self::new(1, 0);
    pub (crate) const SOUTH: Self = Self::new(-1, 0);
    pub (crate) const EAST: Self = Self::new(0, 1);
    pub (crate) const WEST: Self = Self::new(0, -1);
    pub (crate) const NORTH_EAST: Self = Self::new(1, 1);
    pub (crate) const NORTH_WEST: Self = Self::new(1, -1);
    pub (crate) const SOUTH_EAST: Self = Self::new(-1, 1);
    pub (crate) const SOUTH_WEST: Self = Self::new(-1, -1);

    pub (crate) const ORTHOGONAL: [Self; 4] = [
        Direction::NORTH,
        Direction::SOUTH,
        Direction::EAST,
        Direction::WEST,
    ];

    pub (crate) const DIAGONAL: [Self; 4] = [
        Direction::NORTH_EAST,
        Direction::NORTH_WEST,
        Direction::SOUTH_EAST,
        Direction::SOUTH_WEST,
    ];

    pub (crate) const MONARCH: [Self; 8] = [
        Direction::NORTH,
        Direction::SOUTH,
        Direction::EAST,
        Direction::WEST,
        Direction::NORTH_EAST,
        Direction::NORTH_WEST,
        Direction::SOUTH_EAST,
        Direction::SOUTH_WEST,
    ];

    pub (crate) const KNIGHT: [Self; 8] = [
        Direction::new(2, 1),
        Direction::new(2, -1),
        Direction::new(-2, 1),
        Direction::new(-2, -1),
        Direction::new(1, 2),
        Direction::new(-1, 2),
        Direction::new(1, -2),
        Direction::new(-1, -2),
    ];

    pub (crate) const fn new(drank: isize, dfile: isize) -> Self {
        Self {
            drank,
            dfile,
        }
    }

    pub (crate) fn normalize(&self) -> Self {
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
