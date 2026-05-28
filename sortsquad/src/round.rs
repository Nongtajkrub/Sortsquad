use bevy::prelude::*;

use crate::state::RoundState;

#[derive(Resource)]
pub struct RoundCounter(pub u32);

impl Default for RoundCounter {
    fn default() -> Self {
        Self(0)
    }
}

pub fn round_increment(mut round: ResMut<RoundCounter>) {
    round.0 += 1;
}

pub fn setup_round(mut state: ResMut<NextState<RoundState>>) {
    state.set(RoundState::RoundStarting);
}

pub fn start_round(
    mut state: ResMut<NextState<RoundState>>,
) {
    state.set(RoundState::InRound);
}
