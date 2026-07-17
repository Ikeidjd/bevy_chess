use std::marker::PhantomData;

use bevy::{platform::collections::HashSet, prelude::*};

use crate::chess::{direction::Direction, moves::{castling_moves::{CastleBottom, CastleTop}, checks::CheckDetector, move_generator::{CaptureMarker, CaptureOnly, MoveOnly}, pawn_moves::{DoublePawnMoveGenerator, EnPassantMarker}, promotion::PromotingPiece, single_moves::SingleMoveGenerator, sliding_moves::SlidingMoveGenerator}};

pub fn pawn(drank: isize) -> impl Bundle {
    (
        DoublePawnMoveGenerator(Direction::new(drank, 0)),
        MoveOnly(SingleMoveGenerator(HashSet::from([Direction::new(drank, 0)]))),
        CaptureOnly(SingleMoveGenerator(HashSet::from([Direction::new(drank, -1), Direction::new(drank, 1)]))),
        CaptureMarker(SingleMoveGenerator(HashSet::from([Direction::new(drank, -1), Direction::new(drank, 1)])), PhantomData::<EnPassantMarker>),
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
