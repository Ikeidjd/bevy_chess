use std::marker::PhantomData;

use bevy::{platform::collections::HashSet, prelude::*};

use crate::chess::{direction::Direction, markers::EnPassantMarker, moves::{castling_moves::{CastleBottom, CastleTop}, checks::CheckDetector, move_generator::{CaptureOnly, CapturesMarkersAlways, MoveOnly}, pawn_moves::DoublePawnMoveGenerator, promotion::PromotingPiece, single_moves::SingleMoveGenerator, sliding_moves::SlidingMoveGenerator}};

pub (crate) fn pawn(drank: isize) -> impl Bundle {
    (
        DoublePawnMoveGenerator(Direction::new(drank, 0)),
        MoveOnly(SingleMoveGenerator(HashSet::from([Direction::new(drank, 0)]))),
        CaptureOnly(SingleMoveGenerator(HashSet::from([Direction::new(drank, -1), Direction::new(drank, 1)]))),
        CapturesMarkersAlways(PhantomData::<EnPassantMarker>),
        PromotingPiece,
    )
}

pub (crate) fn white_pawn() -> impl Bundle {
    pawn(1)
}

pub (crate) fn black_pawn() -> impl Bundle {
    pawn(-1)
}

pub (crate) fn knight() -> impl Bundle {
    SingleMoveGenerator(HashSet::from(Direction::KNIGHT))
}

pub (crate) fn bishop() -> impl Bundle {
    SlidingMoveGenerator(HashSet::from(Direction::DIAGONAL))
}

pub (crate) fn rook() -> impl Bundle {
    (
        SlidingMoveGenerator(HashSet::from(Direction::ORTHOGONAL)),
        CastleBottom,
    )
}

pub (crate) fn queen() -> impl Bundle {
    SlidingMoveGenerator(HashSet::from(Direction::MONARCH))
}

pub (crate) fn king() -> impl Bundle {
    (
        SingleMoveGenerator(HashSet::from(Direction::MONARCH)),
        CastleTop,
        CheckDetector,
    )
}
