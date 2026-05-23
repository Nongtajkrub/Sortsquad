use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::setup::VIEW_PORT_WIDTH;

use crate::column::Column;

use crate::state::RoundState;
use crate::trashes::TrashKind;
use crate::trashes::Trash;

use crate::items::ItemsYPos;

use crate::player::Player;

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
    window: Query<&Window, With<PrimaryWindow>>,
    trashes: Query<(&TrashKind, &Column), With<Trash>>,
    players: Query<(&TrashKind, &Column), With<Player>>
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
        let Some(pkind) = players
            .iter()
            .find_map(|(k, c)| if c.get() == tcol.get() { Some(k) } else { None })
        else {
            error!("No player in trash column.");
            return;
        };

        if pkind == tkind {
            score.0 += 1;
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
