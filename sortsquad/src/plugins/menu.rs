use bevy::prelude::*;

use crate::menu::titlescreen::titlescreen_button;
use crate::state::GameState;
use crate::state::MenuState;

use crate::menu::setup::setup_menu;
use crate::menu::setup::desetup_menu;

use crate::menu::cutscene::cutscene_play;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<MenuState>()
            .add_systems(OnEnter(GameState::MenuSetup), setup_menu)
            .add_systems(
                Update,
                titlescreen_button.run_if(in_state(MenuState::Titlescreen))
            )
            .add_systems(
                OnEnter(MenuState::Cutscene),
                desetup_menu
            )
            .add_systems(
                Update,
                cutscene_play.run_if(in_state(MenuState::Cutscene))
            );
    }
}
