use bevy::prelude::*;

use bevy::window::PrimaryWindow;
use rand::seq::IndexedRandom;

use crate::setup::VIEW_PORT_WIDTH;

use crate::util::random_bag::RandomBag;

use crate::column::Column;

use crate::player::Player;

#[derive(Resource)]
pub struct TrashImages {
    pub general: Vec<Handle<Image>>,
    pub recycle: Vec<Handle<Image>>,
    pub organic: Vec<Handle<Image>>,
    pub hazardous: Vec<Handle<Image>>,
}

#[derive(Resource)]
pub struct TrashYPos(pub f32);

impl Default for TrashYPos {
    fn default() -> Self {
        Self(0.)
    }
}

#[repr(u8)]
#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
pub enum TrashKind {
    General,
    Recycle,
    Organic,
    Hazardous,
}

impl TrashKind {
    pub fn to_image(&self, assets: &TrashImages) -> Handle<Image> {
        let mut rng = rand::rng();

        // Random the trash itself base on its kind.
        match self {
            TrashKind::General =>
                assets.general
                    .choose(&mut rng)
                    .expect("No general trash assets")
                    .clone(),
            TrashKind::Recycle =>
                assets.recycle
                    .choose(&mut rng)
                    .expect("No recycle trash assets")
                    .clone(),
            TrashKind::Organic =>
                assets.organic
                    .choose(&mut rng)
                    .expect("No organic trash assets")
                    .clone(),
            TrashKind::Hazardous =>
                assets.hazardous
                    .choose(&mut rng)
                    .expect("No hazardous trash assets")
                    .clone(),
        }
    }
}

/// Trash marker
#[derive(Component)]
pub struct Trash;

#[derive(Event)]
pub struct SpawnTrashEvent;

#[derive(Event)]
pub struct ResetTrashEvent;

#[derive(Bundle)]
pub struct TrashBundle {
    trash: Trash,
    col: Column,
    kind: TrashKind,
    transform: Transform,
    sprite: Sprite,
}

pub fn spawn_trashes_observer(
    _trigger: On<SpawnTrashEvent>,
    mut commands: Commands,
    mut ypos: ResMut<TrashYPos>,
    assets: Res<TrashImages>,
    window: Query<&Window, With<PrimaryWindow>>,
    players: Query<(&TrashKind, &Column), (With<Player>, Without<Trash>)>,
) {
    let Ok(window) = window.single() else {
        return;
    };

    ypos.0 = (VIEW_PORT_WIDTH * (window.height() / window.width())) / 2.;

    let mut bag = 
        RandomBag::new(vec![
            TrashKind::General,
            TrashKind::Recycle,
            TrashKind::Organic,
            TrashKind::Hazardous,
        ]);

    for tcol in 0..bag.size() {
        if let Some(pkind) = players
            .iter()
            .find_map(|(kind, col)| {
                if col.get() == tcol as u32 { Some(kind) } else { None }
            })
        {
            let Some(tkind) = bag.try_next_without(*pkind) else {
                error!("TrashKind random bag ran out.");
                return;
            };

            commands.spawn(TrashBundle {
                trash: Trash,
                col: Column::with_size_factor(tcol as u32, 0.7),
                kind: tkind,
                transform: Transform::from_xyz(0., 0., 0.),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0., 0.)),
                    image: tkind.to_image(&assets),
                    ..Default::default()
                }
            });
        }
    }
}

pub fn reset_trashes_observer(
    _trigger: On<ResetTrashEvent>,
    mut commands: Commands,
    trashes: Query<Entity, With<Trash>>
) {
    for entity in trashes {
        commands.entity(entity).despawn();
    }

    commands.trigger(SpawnTrashEvent);
}

pub fn trash_gravity(
    time: Res<Time>,
    mut ypos: ResMut<TrashYPos>,
    mut trashes: Query<&mut Transform, With<Trash>>
) {
    const GRAVITY: f32 = 98.;

    ypos.0 -= GRAVITY * time.delta_secs();
    for mut transform in &mut trashes {
        transform.translation = Vec3::new(transform.translation.x, ypos.0, 0.);
    }
}
