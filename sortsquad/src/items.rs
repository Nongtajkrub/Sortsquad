use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use rand::RngExt;

use crate::setup::VIEW_PORT_WIDTH;

use crate::assets::ImageAssets;
use crate::powerup::Powerup;
use crate::powerup::PowerupKind;
use crate::powerup::PowerupBundle;
use crate::powerup::SpawnPowerupMessage;

use crate::util::random::RandomBag;

use crate::trashes::Trash;
use crate::trashes::TrashKind;
use crate::trashes::TrashImages;
use crate::trashes::TrashBundle;

use crate::column::Column;

use crate::player::Player;

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

#[derive(Event)]
pub struct SpawnItemsEvent;

#[derive(Event)]
pub struct ResetItemsEvent;

pub fn spawn_items_observer(
    _trigger: On<SpawnItemsEvent>,
    mut commands: Commands,
    mut ypos: ResMut<ItemsYPos>,
    mut msg: MessageReader<SpawnPowerupMessage>,
    iassets: Res<ImageAssets>,
    tassets: Res<TrashImages>,
    powerup: Query<&Powerup>,
    window: Query<&Window, With<PrimaryWindow>>,
    players: Query<(&TrashKind, &Column), (With<Player>, Without<Trash>)>,
) {
    let Ok(window) = window.single() else {
        return;
    };

    ypos.0 = (VIEW_PORT_WIDTH * (window.height() / window.width())) / 2.;

    let mut trashes_rng_bag = 
        RandomBag::new(vec![
            TrashKind::General,
            TrashKind::Recycle,
            TrashKind::Organic,
            TrashKind::Hazardous,
        ]);

    // Spawn powerup if have SpawnPowerupMessage and no powerup exist.
    let powerup_col: Option<u32> =
        if msg.read().last().is_some() && powerup.iter().last().is_none() {
            Some(rand::rng().random_range(0..=4))
        } else {
            None
        };

    for tcol in 0..4 {
        // Spawn powerup instead of in powerup round.
        if let Some(powerup_col) = powerup_col {
            if powerup_col == tcol {
                commands.spawn(PowerupBundle {
                    item: Item,
                    powerup: Powerup,
                    col: Column::with_size_factor(tcol as u32, 0.7),
                    kind: PowerupKind::Reveal,
                    transform: Transform::from_xyz(2., 0., 0.),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(0., 0.)),
                        image: iassets.bin_general.clone(),
                        ..Default::default()
                    }
                });

                continue;;
            }
        }

        // Spawn trashes without allowing it to conflict with the player.
        if let Some(pkind) = players
            .iter()
            .find_map(|(kind, col)| {
                if col.get() == tcol as u32 { Some(kind) } else { None }
            })
        {
            let Some(tkind) = trashes_rng_bag.try_next_without(*pkind) else {
                error!("TrashKind random bag ran out.");
                return;
            };

            commands.spawn(TrashBundle {
                item: Item,
                trash: Trash,
                col: Column::with_size_factor(tcol as u32, 0.7),
                kind: tkind,
                transform: Transform::from_xyz(0., 0., 0.),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0., 0.)),
                    image: tkind.to_image_trash(&tassets),
                    ..Default::default()
                }
            });
        }
    }
}

pub fn reset_items_observer(
    _trigger: On<ResetItemsEvent>,
    mut commands: Commands,
    items: Query<Entity, With<Item>>
) {
    for entity in items {
        commands.entity(entity).despawn();
    }

    commands.trigger(SpawnItemsEvent);
}

pub fn items_gravity(
    time: Res<Time>,
    mut ypos: ResMut<ItemsYPos>,
    mut items: Query<&mut Transform, With<Item>>
) {
    const GRAVITY: f32 = 98.;

    ypos.0 -= GRAVITY * time.delta_secs();
    for mut transform in &mut items {
        transform.translation = Vec3::new(transform.translation.x, ypos.0, 0.);
    }
}
