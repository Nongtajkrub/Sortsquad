use bevy::prelude::*;

pub mod util; 
pub mod player;
pub mod trash;

mod setup;

use setup::setup;

use player::move_general_player;
use player::move_recycle_player;
use player::move_hazardous_player;
use player::move_organic_player;

use util::column::column_sync;

use trash::spawn_trashes;
use trash::trash_gravity;

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
        .add_systems(Update, trash_gravity)
        .add_observer(column_sync)
        .add_observer(spawn_trashes)
        .run();
}
