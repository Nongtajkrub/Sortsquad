use bevy::prelude::*;

use crate::state::GameState;

use crate::end::setup::setup_end;

pub struct EndPlugin;

impl Plugin for EndPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Ended), setup_end);
    }
}
