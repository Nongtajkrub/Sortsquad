use bevy::prelude::*;

pub mod util; 
pub mod player;
pub mod trash;

mod setup;

use setup::setup;

use player::move_players;

use util::column::column_sync;

use trash::spawn_trashes;
use trash::trash_gravity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, move_players)
        .add_systems(Update, trash_gravity)
        .add_observer(column_sync)
        .add_observer(spawn_trashes)
        .run();
}
