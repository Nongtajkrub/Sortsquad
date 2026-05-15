use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use rand::seq::IndexedRandom;

use crate::util::random_bag::RandomBag;

use crate::column::Column;

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
#[derive(Component, PartialEq, Eq)]
pub enum TrashKind {
    General,
    Recycle,
    Organic,
    Hazardous,
}

impl TrashKind {
    pub fn to_sprite(&self, assets: &TrashImages) -> Sprite {
        let mut rng = rand::rng();

        // Random the trash itself base on its kind.
        match self {
            TrashKind::General =>
                Sprite::from_image(
                    assets.general
                        .choose(&mut rng)
                        .expect("No general trash assets")
                        .clone()
                ),
            TrashKind::Recycle =>
                Sprite::from_image(
                    assets.recycle
                        .choose(&mut rng)
                        .expect("No recycle trash assets")
                        .clone()
                ),
            TrashKind::Organic =>
                Sprite::from_image(
                    assets.organic
                        .choose(&mut rng)
                        .expect("No organic trash assets")
                        .clone()
                ),
            TrashKind::Hazardous =>
                Sprite::from_image(
                    assets.hazardous
                        .choose(&mut rng)
                        .expect("No hazardous trash assets")
                        .clone()
                ),
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
) {
    let Ok(window) = window.single() else {
        return;
    };

    let top_edge = window.height() / 2.;

    ypos.0 = top_edge;

    let mut bag = 
        RandomBag::new(vec![
            TrashKind::General,
            TrashKind::Recycle,
            TrashKind::Organic,
            TrashKind::Hazardous]
        );
    
    for i in 0..bag.size() {
        let kind = bag.next().expect("Random bag ran out of trash kind");

        commands.spawn(
            TrashBundle {
                trash: Trash,
                col: Column::with_size_factor(i as u32, 0.5),
                sprite: kind.to_sprite(&assets),
                kind: kind,
                transform: Transform::from_xyz(0., top_edge, 0.),
            }
        );
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
