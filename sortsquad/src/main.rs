use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub mod util; 
pub mod state;
pub mod configs;
pub mod assets;

mod game;
mod menu;
mod plugins;
mod setup;

use crate::setup::setup_main;

use crate::state::GameState;

use crate::assets::ImageAssets;
use crate::assets::FontAssets;

use crate::plugins::game::GamePlugin;

use crate::plugins::menu::MenuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetsLoading)
                .continue_to_state(GameState::MenuSetup)
                .load_collection::<ImageAssets>()
                .load_collection::<FontAssets>()
        )
        .add_systems(Startup, setup_main)
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .run();
}
