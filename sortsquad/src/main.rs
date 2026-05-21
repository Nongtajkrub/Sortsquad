pub mod setup;
pub mod util; 
pub mod player;
pub mod trashes;
pub mod items;
pub mod score;
pub mod powerup;
pub mod round;
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

use crate::powerup::powerup_spawner;
use crate::powerup::powerup_despawn_observer;
use crate::powerup::SpawnPowerupMessage;

use crate::round::RoundCounter;
use crate::round::round_increment_observer;

use crate::player::players_move;
use crate::player::players_apply_collector_effect;
use crate::player::players_remove_collector_effect;
use crate::player::players_sync_control_label;

use crate::items::ItemsYPos;
use crate::items::spawn_items_observer;
use crate::items::reset_items_observer;
use crate::items::items_gravity;

use score::Score;
use score::scoring;
use score::sync_score_text;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<Score>()
        .init_resource::<ItemsYPos>()
        .init_resource::<RoundCounter>()
        .init_state::<GameState>()
        .add_message::<SpawnPowerupMessage>()
        .add_loading_state(
            LoadingState::new(GameState::AssetsLoading)
                .continue_to_state(GameState::Starting)
                .load_collection::<ImageAssets>()
                .load_collection::<FontAssets>()
        )
        .add_observer(spawn_items_observer)
        .add_observer(reset_items_observer)
        .add_observer(round_increment_observer)
        .add_observer(powerup_despawn_observer)
        .add_systems(OnEnter(GameState::Starting), setup_game)
        .add_systems(
            Update,
            (
                players_move,
                players_apply_collector_effect,
                players_remove_collector_effect,
                players_sync_control_label,
                items_gravity,
                scoring,
                powerup_spawner,
                column_sync,
                achor_bottom_sync,
                sync_score_text
                    .run_if(resource_changed::<Score>),
            ).run_if(in_state(GameState::Playing)))
        .run();
}
