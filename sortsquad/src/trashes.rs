use bevy::prelude::*;

use crate::items::Item;

use rand::seq::IndexedRandom;

use crate::assets::ImageAssets;

use crate::column::Column;

#[derive(Resource)]
pub struct TrashImages {
    pub general: Vec<Handle<Image>>,
    pub recycle: Vec<Handle<Image>>,
    pub organic: Vec<Handle<Image>>,
    pub hazardous: Vec<Handle<Image>>,
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
    pub fn to_image_trash(&self, assets: &TrashImages) -> Handle<Image> {
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

    pub fn to_image_player(&self, assets: &ImageAssets) -> Handle<Image> {
        match self {
            TrashKind::General => assets.bin_general.clone(),
            TrashKind::Recycle => assets.bin_recycle.clone(),
            TrashKind::Organic => assets.bin_organic.clone(),
            TrashKind::Hazardous => assets.bin_hazardous.clone(),
        }
    }
}

/// Trash marker
#[derive(Component)]
pub struct Trash;

#[derive(Bundle)]
pub struct TrashBundle {
    pub item: Item,
    pub trash: Trash,
    pub col: Column,
    pub kind: TrashKind,
    pub transform: Transform,
    pub sprite: Sprite,
}
