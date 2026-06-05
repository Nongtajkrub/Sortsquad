use bevy::prelude::*;

use crate::util::achor::SpriteAchorBottom;

use crate::assets::ImageAssets;

use crate::game::column::Column;

use crate::game::trashes::TrashKind;
use crate::game::trashes::Trash;

use crate::game::powerup::PowerupKind;
use crate::game::powerup::ActivePowerup;

/// Player marker
#[derive(Component)]
pub struct Player;

/// Mark players who can collect power ups.
#[derive(Component)]
pub struct PlayerPowerCollector;

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
    pub achor: SpriteAchorBottom,
}

/// System for moving the general bin player.
pub fn players_move(
    keyboard: Res<ButtonInput<KeyCode>>, 
    active: Res<ActivePowerup>,
    mut players: Query<
        (Entity, &mut Column, &TrashKind, &PlayerControl),
        With<Player>
    >,
    trashes: Query<(&Column, &TrashKind), (With<Trash>, Without<Player>)>
) {
    if active.0 == PowerupKind::AutoCorrect {
        for (_, mut pcol, pkind, _) in &mut players {
            if let Some(tcol) = trashes
                .iter()
                .find_map(|(col, kind)| {
                    if kind == pkind { Some(col.get()) } else { None }
                })
            {
               pcol.set(tcol); 
            }
        }

        return;
    }

    let mut intended_swaps: Vec<(Entity, i32)> = Vec::new();

    for (entity, col, _, control) in players.iter() {
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
            .find_map(|(entity, col, _, _)| {
                if col.get() as i32 == target_col { Some(entity) } else { None }
            })
        {
            let [(_, mut mover_col, _, _), (_, mut target_col, _, _)] = players
                .get_many_mut([mover, target])
                .expect("Fail to retrive mover or target");

            // Swap their column.
            let tmp = mover_col.get();
            mover_col.set(target_col.get());
            target_col.set(tmp);
        }
    }
}

pub fn players_apply_collector_effect(
    assets: Res<ImageAssets>,
    mut player: Query<&mut Sprite, (With<Player>, Added<PlayerPowerCollector>)>
) {
    let Ok(mut sprite) = player.single_mut() else {
        return;
    };

    sprite.image = assets.bin_powerup.clone();
}

pub fn players_remove_collector_effect(
    assets: Res<ImageAssets>,
    mut players: RemovedComponents<PlayerPowerCollector>,
    mut sprites: Query<(&mut Sprite, &TrashKind), With<Player>>,
) {
    if players.len() > 1 {
        error!("Only one player should have the PlayerPowerCollector component");
    }

    for player in &mut players.read() {
        if let Ok((mut sprite, kind)) = sprites.get_mut(player) {
            sprite.image = kind.to_image_player(&assets);
        }
    }
}

pub fn players_sync_control_label(
    mut labels: Query<
        (&PlayerControlLabel, &mut Column, &mut Transform),
        Without<Player>
    >,
    players: Query<(&Column, &Sprite, &Transform), With<Player>>,
) {
    for (control_label, mut lcol, mut ltrans) in &mut labels {
        if let Ok((pcol, sprite, ptrans)) = players.get(control_label.0) {
            if let Some(size) = sprite.custom_size {
                ltrans.translation.y = ptrans.translation.y + (size.y / 2.);
            }

            lcol.set(pcol.get());
        }
    }
}
