use bevy::prelude::*;

pub mod util; 
pub mod player;

mod setup;

use setup::setup;

use player::move_general_player;
use player::move_recycle_player;
use player::move_hazardous_player;
use player::move_organic_player;
use player::sync_player_slot;

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
        .add_observer(sync_player_slot)
        .run();
}
