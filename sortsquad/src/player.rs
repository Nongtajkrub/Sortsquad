use bevy::prelude::*;

use crate::util::align::Align;

use crate::column::Column;
use crate::column::ColumnResyncEvent;

use crate::trash::TrashKind;

/// Player marker
#[derive(Component)]
pub struct Player;

// Player control label marker.
#[derive(Component)]
pub struct PlayerControlLabel(pub Entity);

#[derive(Component)]
pub struct PlayerControl {
    pub left: KeyCode,
    pub right: KeyCode,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub kind: TrashKind,
    pub col: Column,
    pub control: PlayerControl,
    pub transform: Transform,
    pub sprite: Sprite,
    pub align: Align,
}

/// System for moving the general bin player.
pub fn move_players(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>, 
    mut players: Query<(Entity, &mut Column, &PlayerControl), With<Player>>,
) {
    let mut intended_swaps: Vec<(Entity, i32)> = Vec::new();

    for (entity, col, control) in players.iter() {
        if keyboard.just_pressed(control.left) {
            intended_swaps.push((entity, col.get() as i32 - 1));
        } else if keyboard.just_pressed(control.right) {
            intended_swaps.push((entity, col.get() as i32 + 1));
        } else {
            continue;
        }
    }
    for (mover, target_col) in intended_swaps {
        if let Some(target) = players
            .iter_mut()
            .find_map(|(entity, col, _)| {
                if col.get() as i32 == target_col { Some(entity) } else { None }
            })
        {
            let [(_, mut mover_col, _), (_, mut target_col, _)] = players
                .get_many_mut([mover, target])
                .expect("Fail to retrive mover or target");

            // Swap their column.
            let tmp = mover_col.get();
            mover_col.set(target_col.get());
            target_col.set(tmp);

            commands.trigger(ColumnResyncEvent);
        }
    }
}

pub fn sync_player_control_label(
    mut labels: Query<(&PlayerControlLabel, &mut Column), Without<Player>>,
    players: Query<&Column, (With<Player>, Changed<Column>)>,
) {
    // No player column changed.
    if players.iter().last().is_none() {
        return;
    }

    for (control_label, mut lcol) in &mut labels {
        if let Ok(pcol) = players.get(control_label.0) {
            lcol.set(pcol.get());
        }
    }
}
