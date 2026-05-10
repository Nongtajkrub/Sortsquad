use bevy::prelude::*;

use crate::util::column::Column;
use crate::util::column::ColumnResyncEvent;

use crate::trash::TrashKind;

/// Player marker
#[derive(Component)]
pub struct Player;

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
            intended_swaps.push((entity, col.n as i32 - 1));
        } else if keyboard.just_pressed(control.right) {
            intended_swaps.push((entity, col.n as i32 + 1));
        } else {
            continue;
        }
    }

    for (mover, target_col) in intended_swaps {
        if let Some(target) = players
            .iter_mut()
            .find_map(|(entity, col, _)| {
                if col.n as i32 == target_col { Some(entity) } else { None }
            })
        {
            let [(_, mut mover_col, _), (_, mut target_col, _)] = players
                .get_many_mut([mover, target])
                .expect("Fail to retrive mover or target");

            std::mem::swap(&mut mover_col.n, &mut target_col.n);
            commands.trigger(ColumnResyncEvent);
        }
    }
}
