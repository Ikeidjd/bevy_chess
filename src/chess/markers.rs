use bevy::prelude::*;

#[derive(Component, Default, Clone)]
pub struct MarkerBoard {
    pub current: Vec<Entity>,
    pub future: Vec<Entity>,
}

impl MarkerBoard {
    pub fn insert(&mut self, marker: Entity) {
        self.future.push(marker);
    }

    pub fn advance_move(&mut self) {
        self.current = std::mem::take(&mut self.future);
    }

    pub fn remove_future_markers(&mut self) {
        self.future.clear();
    }
}


// A PieceMarker is used to signal that a piece can be captured from a position other than its own
// This is used for double pawn moves (en passant) and castling (the king can't move out of or through check, i.e., it can be captured even though that is not its position)
pub trait PieceMarker {
    fn get_entity(&self) -> Entity;
}

// Used for despawning markers
// Make every component that implements PieceMarker #require this or bad things will happen
#[derive(Component, Default)]
pub struct PieceMarkerRequire;
