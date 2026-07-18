use bevy::{input::keyboard::Key, prelude::*};

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
#[derive(Component)]
pub struct PieceMarkerRequire {
    pub sprite_name: &'static str,
}

pub struct MarkerVisibilityPlugin;

impl Plugin for MarkerVisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MarkerVisibilityState>()
            .add_systems(Update, (update_marker_visibility_state, add_marker_sprites, update_marker_visibilities));
    }
}

#[derive(States, Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
struct MarkerVisibilityState(bool);

impl MarkerVisibilityState {
    fn toggled(&self) -> Self {
        Self(!self.0)
    }
}

fn update_marker_visibility_state(state: Res<State<MarkerVisibilityState>>, mut next_state: ResMut<NextState<MarkerVisibilityState>>, input: Res<ButtonInput<Key>>) {
    if input.just_released(Key::F1) {
        next_state.set(state.toggled());
    }
}

fn add_marker_sprites(mut commands: Commands, asset_server: Res<AssetServer>, markers: Query<(Entity, &PieceMarkerRequire), Without<Sprite>>) {
    for (marker_entity, marker) in markers {
        commands.entity(marker_entity).insert((
            Sprite::from_image(asset_server.load(marker.sprite_name)),
            Visibility::Hidden,
        ));
    }
}

fn update_marker_visibilities(marker_board: Single<&MarkerBoard>, state: Res<State<MarkerVisibilityState>>, mut marker_visibilities: Query<&mut Visibility, With<PieceMarkerRequire>>) {
    for mut visibility in &mut marker_visibilities {
        *visibility = Visibility::Hidden;
    }

    if !state.0 {
        return;
    }

    for &marker in &marker_board.current {
        let mut visibility = marker_visibilities.get_mut(marker).unwrap();
        *visibility = Visibility::Visible;
    }
}
