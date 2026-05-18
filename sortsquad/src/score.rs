use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::setup::VIEW_PORT_WIDTH;

use crate::column::Column;

use crate::trash::TrashYPos;
use crate::trash::TrashKind;
use crate::trash::Trash;
use crate::trash::ResetTrashEvent;

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
    mut commands: Commands,
    mut score: ResMut<Score>,
    ypos: Res<TrashYPos>,
    window: Query<&Window, With<PrimaryWindow>>,
    trashes: Query<(&TrashKind, &Column), With<Trash>>,
    players: Query<(&TrashKind, &Column), With<Player>>
) {
    let Ok(window) = window.single() else {
        return;
    };

    let bottom_edge =
        -((VIEW_PORT_WIDTH * (window.height() / window.width())) / 2.);

    if ypos.0 > bottom_edge + 100. {
        return;
    }

    println!("Y pos: {}, Bottom: {}", ypos.0, bottom_edge);
    
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

    commands.trigger(ResetTrashEvent);
}

pub fn sync_score_text(
    mut text: Single<&mut Text, With<ScoreText>>,
    score: Res<Score>
) {
    text.0 = format!("{}", score.0);
}
