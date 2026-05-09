use bevy::prelude::*;

pub mod player;

mod setup;

use setup::setup;

use player::move_general_player;
use player::move_recycle_player;

use crate::player::move_hazardous_player;
use crate::player::move_organic_player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            move_general_player,
            move_recycle_player,
            move_organic_player,
            move_hazardous_player
        ))
        .run();
}
