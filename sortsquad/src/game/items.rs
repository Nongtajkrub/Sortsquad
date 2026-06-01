use bevy::prelude::*;

use rand::RngExt;

use crate::util::random::random_from_list;
use crate::util::random::RandomBag;

use crate::assets::ImageAssets;

use crate::game::round::RoundCounter;

use crate::game::column::Column;

use crate::game::powerup::Powerup;
use crate::game::powerup::ActivePowerup;
use crate::game::powerup::PowerupKind;
use crate::game::powerup::PowerupBundle;

use crate::game::trashes::Trash;
use crate::game::trashes::TrashKind;
use crate::game::trashes::TrashImages;
use crate::game::trashes::TrashBundle;

use crate::game::player::Player;
use crate::game::player::PlayerPowerCollector;

// Item marker.
#[derive(Component)]
pub struct Item;

#[derive(Resource)]
pub struct ItemsYPos(pub f32);

impl Default for ItemsYPos {
    fn default() -> Self {
        Self(0.)
    }
}

pub fn spawn_items(
    mut commands: Commands,
    round: Res<RoundCounter>,
    iassets: Res<ImageAssets>,
    tassets: Res<TrashImages>,
    active: Res<ActivePowerup>,
    powerup: Query<&Powerup>,
    players: Query<(Entity, &TrashKind, &Column), With<Player>>,
) {
    let mut powerup_info: Option<(TrashKind, u32)> = None;

    // Only spawn power up every specific round if it does not exist.
    if round.0 > 0 && round.0 % 2 == 0 && powerup.iter().last().is_none() {
        let collector_kind = random_from_list(&TrashKind::ALL);

        let Some(collector) = players
            .iter()
            .find_map(|(entity, pkind, _)| {
                if *pkind == collector_kind { Some(entity) } else { None }
            })
        else {
            error!("Can't find player to apply power collector to.");
            return;
        };

        commands.entity(collector).insert(PlayerPowerCollector);
        powerup_info = Some((collector_kind, rand::rng().random_range(0..4)));
    }

    let mut trashes_rng_bag = RandomBag::new(
        TrashKind::ALL
            .into_iter()
            .filter(|kind| !powerup_info.is_some_and(|(ckind, _)| ckind == *kind))
            .collect::<Vec<TrashKind>>()
    );

    for col in 0..4 {
        // Spawn powerup instead of in powerup round.
        if let Some((_, powerup_col)) = powerup_info {
            if powerup_col == col {
                commands.spawn(PowerupBundle {
                    item: Item,
                    powerup: Powerup,
                    col: Column::with_size_factor(col as u32, 0.7),
                    transform: Transform::from_xyz(0., 0., 0.),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(0., 0.)),
                        image: iassets.bin_general.clone(),
                        ..Default::default()
                    }
                });

                continue;
            }
        }

        // Spawn trashes without allowing it to conflict with the player.
        if let Some(pkind) = players
            .iter()
            .find_map(|(_, kind, pcol)| {
                if pcol.get() == col as u32 { Some(kind) } else { None }
            })
        {
            let Some(tkind) = trashes_rng_bag.try_next_without(*pkind) else {
                error!("TrashKind random bag ran out.");
                return;
            };

            let tint = if active.0 == PowerupKind::Highlight {
                tkind.to_tint()
            } else {
                Color::srgb(1., 1., 1.)
            };

            commands.spawn(TrashBundle {
                item: Item,
                trash: Trash,
                col: Column::with_size_factor(col as u32, 0.7),
                kind: tkind,
                transform: Transform::from_xyz(0., 0., 0.),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0., 0.)),
                    image: tkind.to_image_trash(&tassets),
                    color: tint,
                    ..Default::default()
                }
            });
        }
    }
}

pub fn despawn_items(
    mut commands: Commands,
    items: Query<Entity, With<Item>>
) {
    for entity in items {
        commands.entity(entity).despawn();
    }
}

pub fn items_gravity(
    time: Res<Time>,
    active: Res<ActivePowerup>,
    mut ypos: ResMut<ItemsYPos>,
    mut items: Query<&mut Transform, With<Item>>
) {
    let gravity: f32 = if active.0 == PowerupKind::SlowDown {
        64.
    } else {
        98.
    };

    ypos.0 -= gravity * time.delta_secs();
    for mut transform in &mut items {
        transform.translation = Vec3::new(transform.translation.x, ypos.0, 0.);
    }
}
