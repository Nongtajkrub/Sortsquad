use bevy::prelude::*;

use crate::menu::setup::setup_menu;
use crate::state::{GameState, MenuState};

use crate::menu::cutscene::cutscene_play;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<MenuState>()
            .add_systems(OnEnter(GameState::MenuSetup), setup_menu)
            .add_systems(
                Update,
                cutscene_play.run_if(in_state(MenuState::Cutscene))
            );
    }
}
