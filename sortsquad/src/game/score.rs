use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::util::random::random_from_list;

use crate::setup::VIEW_PORT_WIDTH;

use crate::state::RoundState;

use crate::game::column::Column;

use crate::game::items::ItemsYPos;

use crate::game::trashes::TrashKind;
use crate::game::trashes::Trash;

use crate::game::powerup::Powerup;
use crate::game::powerup::PowerupKind;
use crate::game::powerup::ActivePowerup;

use crate::game::player::PlayerPowerCollector;
use crate::game::player::Player;

#[derive(Resource)]
pub struct Score(pub u32);

impl Default for Score {
    fn default() -> Self {
        Self (0)
    }
}

// Score text marker.
#[derive(Component)]
pub struct ScoreText;

pub fn scoring(
    mut score: ResMut<Score>,
    mut state: ResMut<NextState<RoundState>>,
    mut ypos: ResMut<ItemsYPos>,
    mut active: ResMut<ActivePowerup>,
    window: Query<&Window, With<PrimaryWindow>>,
    trashes: Query<(&TrashKind, &Column), With<Trash>>,
    players: Query<
        (&TrashKind, &Column),
        (With<Player>, Without<PlayerPowerCollector>)
    >,
    powerup: Query<&Column, With<Powerup>>,
    pcollector: Query<&Column, With<PlayerPowerCollector>>
) {
    let Ok(window) = window.single() else {
        return;
    };

    let top_edge =
        (VIEW_PORT_WIDTH * (window.height() / window.width())) / 2.;
    let bottom_edge =
        -((VIEW_PORT_WIDTH * (window.height() / window.width())) / 2.);

    if ypos.0 > bottom_edge + 100. {
        return;
    }
    
    for (tkind, tcol) in &trashes {
        if let Some(pkind) = players
            .iter()
            .find_map(|(k, c)| if c.get() == tcol.get() { Some(k) } else { None }) 
        {
            if pkind == tkind {
                score.0 += if active.0 == PowerupKind::DoublePoint {
                    2 
                } else {
                    1
                }
            }
        };
    }

    active.0 = PowerupKind::NoPowerup;

    if let Ok(ccol) = pcollector.single() && let Ok(pcol) = powerup.single() {
        if ccol.get() == pcol.get() {
            active.0 = random_from_list(&PowerupKind::ALL);
        }
    }

    ypos.0 = top_edge;
    state.set(RoundState::RoundEnding);
}

pub fn sync_score_text(
    mut text: Single<&mut Text, With<ScoreText>>,
    score: Res<Score>
) {
    text.0 = format!("{}", score.0);
}
