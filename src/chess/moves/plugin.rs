use bevy::prelude::*;

use crate::chess::moves::{single_moves::SingleMoveGeneratorPlugin, sliding_moves::SlidingMoveGeneratorPlugin};

pub (crate) struct MovesPlugin;

impl Plugin for MovesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SingleMoveGeneratorPlugin, SlidingMoveGeneratorPlugin));
    }
}
