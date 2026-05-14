use bevy::prelude::*;

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
    window: Query<&Window>,
    trashes: Query<(&TrashKind, &Column), With<Trash>>,
    players: Query<(&TrashKind, &Column), With<Player>>
) {
    let Ok(window) = window.single() else {
        return;
    };

    if ypos.0 > -(window.height() / 2.) + 100. {
        return;
    }
    
    for (tkind, tcol) in &trashes {
        let pkind = players
            .iter()
            .find_map(|(k, c)| if c.get() == tcol.get() { Some(k) } else { None })
            .expect("No player in trash column.");

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
