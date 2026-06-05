use std::time::Duration;

use bevy::prelude::*;

use crate::configs::GAME_TIME_MS;

use crate::state::GameState;
use crate::state::RoundState;

#[derive(Component)]
pub struct TimerText;

#[derive(Resource)]
pub struct GameTimer(pub Timer);

impl Default for GameTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(GAME_TIME_MS), TimerMode::Once))
    }
}

pub fn timer_update(
    mut timer: ResMut<GameTimer>,
    time: Res<Time>,
    mut text: Query<(&mut Text2d, &mut TextColor), With<TimerText>>,
    mut gstate: ResMut<NextState<GameState>>,
    mut rstate: ResMut<NextState<RoundState>>
) {
    let Ok((mut text, mut color)) = text.single_mut() else {
        error!("Only one timer text should exist.");
        return;
    };

    timer.0.tick(time.delta());

    if timer.0.is_finished() {
        rstate.set(RoundState::NoMoreRound);
        gstate.set(GameState::Ended);
    }

    **text = format!("{}", timer.0.remaining_secs().round()); 

    let update_color =
        timer.0.remaining().as_millis() as f32 / GAME_TIME_MS as f32;
    color.0 = Color::srgba(1., update_color, update_color, 0.6);
}
