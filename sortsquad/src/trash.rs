use bevy::prelude::*;

use rand::seq::IndexedRandom;

use crate::util::random_bag::RandomBag;
use crate::util::column::Column;
use crate::util::column::ColumnResyncEvent;

#[derive(Resource)]
pub struct TrashImages {
    pub general: Vec<Handle<Image>>,
    pub recycle: Vec<Handle<Image>>,
    pub organic: Vec<Handle<Image>>,
    pub hazardous: Vec<Handle<Image>>,
}

#[repr(u8)]
#[derive(Component)]
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

#[derive(Bundle)]
pub struct TrashBundle {
    trash: Trash,
    col: Column,
    kind: TrashKind,
    transform: Transform,
    sprite: Sprite,
}

pub fn spawn_trashes(
    _trigger: On<SpawnTrashEvent>,
    mut commands: Commands,
    assets: Res<TrashImages>
) {
    let mut bag = 
        RandomBag::new(vec![
            TrashKind::General,
            TrashKind::Recycle,
            TrashKind::Organic,
            TrashKind::Hazardous]
        );
    
    for i in 0..bag.size() {
        let kind = bag.next().expect("Random bag ran out of trash kind.");

        commands.spawn(
            TrashBundle {
                trash: Trash,
                col: Column::with_size_factor(i as u32, 0.5),
                sprite: kind.to_sprite(&assets),
                kind: kind,
                transform: Transform::from_xyz(0., 0., 0.),
            }
        );
    }

    commands.trigger(ColumnResyncEvent);
}

pub fn trash_gravity(
    time: Res<Time>,
    mut trashes: Query<&mut Transform, With<Trash>>
) {
    const GRAVITY: f32 = 24.;

    for mut transform in &mut trashes {
        transform.translation -= Vec3::new(0., GRAVITY * time.delta_secs(), 0.);
    }
}
