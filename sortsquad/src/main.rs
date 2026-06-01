use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub mod util; 
pub mod state;
pub mod assets;
pub mod setup;

mod game;
mod plugins;

use crate::state::GameState;

use crate::setup::setup_game;

use crate::assets::ImageAssets;
use crate::assets::FontAssets;

use crate::plugins::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetsLoading)
                .continue_to_state(GameState::Starting)
                .load_collection::<ImageAssets>()
                .load_collection::<FontAssets>()
        )
        .add_systems(OnEnter(GameState::Starting), setup_game)
        .add_plugins(GamePlugin)
        .run();
}
