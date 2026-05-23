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
pub mod state;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use setup::setup_game;

use util::achor::achor_bottom_sync;

use state::GameState;
use crate::state::RoundState;

use assets::ImageAssets;
use assets::FontAssets;

use crate::column::column_sync;

use crate::items::spawn_items;
use crate::items::despawn_items;
use crate::items::ItemsYPos;
use crate::items::items_gravity;

use crate::powerup::powerup_despawn;

use crate::round::setup_round;
use crate::round::start_round;
use crate::round::RoundCounter;
use crate::round::round_increment;

use crate::player::players_move;
use crate::player::players_apply_collector_effect;
use crate::player::players_remove_collector_effect;
use crate::player::players_sync_control_label;

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
        .init_state::<RoundState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetsLoading)
                .continue_to_state(GameState::Starting)
                .load_collection::<ImageAssets>()
                .load_collection::<FontAssets>()
        )
        .add_systems(OnEnter(GameState::Starting), setup_game)
        .add_systems(
            OnEnter(RoundState::RoundStarting),
            (spawn_items, start_round).chain()
        )
        .add_systems(
            OnEnter(RoundState::RoundEnding),
            (
                round_increment,
                powerup_despawn,
                despawn_items,
                setup_round
            ).chain()
        )
        .add_systems(
            Update,
            (
                players_move,
                players_apply_collector_effect,
                players_remove_collector_effect,
                players_sync_control_label,
                items_gravity,
                column_sync,
                achor_bottom_sync,
                scoring.run_if(resource_changed::<ItemsYPos>),
                sync_score_text.run_if(resource_changed::<Score>),
            ).run_if(in_state(GameState::Playing)))
        .run();
}
