pub mod setup;
pub mod util; 
pub mod player;
pub mod trash;
pub mod score;
pub mod column;
pub mod assets;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use setup::setup_game;

use util::achor::achor_bottom_sync;

use assets::GameState;
use assets::ImageAssets;
use assets::FontAssets;

use crate::column::column_sync;
use crate::player::sync_player_control_label;

use player::move_players;

use crate::trash::TrashYPos;
use trash::spawn_trashes_observer;
use trash::reset_trashes_observer;
use trash::trash_gravity;

use score::Score;
use score::scoring;
use score::sync_score_text;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<Score>()
        .init_resource::<TrashYPos>()
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetsLoading)
                .continue_to_state(GameState::Starting)
                .load_collection::<ImageAssets>()
                .load_collection::<FontAssets>()
        )
        .add_systems(OnEnter(GameState::Starting), setup_game)
        .add_systems(
            Update,
            (
                move_players,
                trash_gravity,
                scoring,
                column_sync,
                achor_bottom_sync,
                sync_player_control_label,
                sync_score_text.run_if(resource_changed::<Score>),
            ).run_if(in_state(GameState::Playing)))
        .add_observer(spawn_trashes_observer)
        .add_observer(reset_trashes_observer)
        .run();
}
