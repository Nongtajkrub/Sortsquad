use bevy::prelude::*;

use crate::state::GameState;
use crate::state::MenuState;

use crate::menu::cutscene::CutsceneSequence;

pub fn setup_menu(
    mut commands: Commands,
    mut gstate: ResMut<NextState<GameState>>,
    mut mstate: ResMut<NextState<MenuState>>
) {
    commands.init_resource::<CutsceneSequence>();

    gstate.set(GameState::Menu);
    mstate.set(MenuState::Cutscene);
}
