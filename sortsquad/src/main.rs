pub mod util; 
pub mod player;
pub mod trash;
pub mod score;
pub mod column;
pub mod assets;
mod setup;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use setup::setup;

use assets::GameState;

use crate::assets::ImageAssets;
use crate::util::align::align_sync;

use player::move_players;

use column::column_sync;

use trash::spawn_trashes;
use trash::reset_trashes;
use trash::trash_gravity;

use score::scoring;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetsLoading)
                .continue_to_state(GameState::Starting)
                .load_collection::<ImageAssets>()
        )
        .add_systems(OnEnter(GameState::Starting), setup)
        .add_systems(
            Update,
            (
                move_players,
                trash_gravity,
                scoring,
                align_sync
            ).run_if(in_state(GameState::Playing)))
        .add_observer(spawn_trashes)
        .add_observer(reset_trashes)
        .add_observer(column_sync)
        .run();
}
