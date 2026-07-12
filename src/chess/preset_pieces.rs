use bevy::{platform::collections::HashSet, prelude::*};

use crate::chess::{direction::Direction, moves::{CaptureGenerator, castling_moves::{CastleBottom, CastleTop}, checks::CheckDetector, pawn_moves::PawnMoveGenerator, promotion::PromotingPiece, single_moves::SingleMoveGenerator, sliding_moves::SlidingMoveGenerator}};

pub fn pawn(drank: isize) -> impl Bundle {
    (
        PawnMoveGenerator(Direction::new(drank, 0)),
        CaptureGenerator(SingleMoveGenerator(HashSet::from([Direction::new(drank, -1), Direction::new(drank, 1)]))),
        PromotingPiece,
    )
}

pub fn white_pawn() -> impl Bundle {
    pawn(1)
}

pub fn black_pawn() -> impl Bundle {
    pawn(-1)
}

pub fn knight() -> impl Bundle {
    SingleMoveGenerator(HashSet::from(Direction::KNIGHT))
}

pub fn bishop() -> impl Bundle {
    SlidingMoveGenerator(HashSet::from(Direction::DIAGONAL))
}

pub fn rook() -> impl Bundle {
    (
        SlidingMoveGenerator(HashSet::from(Direction::ORTHOGONAL)),
        CastleBottom,
    )
}

pub fn queen() -> impl Bundle {
    SlidingMoveGenerator(HashSet::from(Direction::MONARCH))
}

pub fn king() -> impl Bundle {
    (
        SingleMoveGenerator(HashSet::from(Direction::MONARCH)),
        CastleTop,
        CheckDetector,
    )
}
