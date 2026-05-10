use bevy::prelude::*;

use crate::player::Player;
use crate::player::PlayerBundle;
use crate::player::GeneralPlayer;
use crate::player::OrganicPlayer;
use crate::player::RecyclePlayer;
use crate::player::HazardousPlayer;

use crate::trash::SpawnTrashEvent;
use crate::trash::TrashImages;
use crate::util::column::Column;

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player,
        GeneralPlayer,
        PlayerBundle {
            col: Column::new(0),
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::from_image(assets.load("bins/general/static.png")),
        }
    ));

    commands.spawn((
        Player,
        RecyclePlayer,
        PlayerBundle {
            col: Column::new(1),
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::from_image(assets.load("bins/recyclable/static.png")),
        }
    ));

    commands.spawn((
        Player,
        OrganicPlayer,
        PlayerBundle {
            col: Column::new(2),
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::from_image(assets.load("bins/organic/static.png")),
        }
    ));

    commands.spawn((
        Player,
        HazardousPlayer,
        PlayerBundle {
            col: Column::new(3),
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::from_image(assets.load("bins/hazardous/static.png")),
        }
    ));

    commands.insert_resource(TrashImages {
        general: vec![
            assets.load("trashes/general/ciggarette.png"),
            assets.load("trashes/general/shoe.png"),
            assets.load("trashes/general/tissue.png")
        ],
        recycle: vec![
            assets.load("trashes/recyclable/coke.png"),
            assets.load("trashes/recyclable/newspaper.png"),
            assets.load("trashes/recyclable/waterbottle.png"),
        ],
        organic: vec![
            assets.load("trashes/organic/apple.png"),
            assets.load("trashes/organic/fishbone.png"),
            assets.load("trashes/organic/vegatable.png"),
        ],
        hazardous: vec![
            assets.load("trashes/hazardous/battery.png"),
            assets.load("trashes/hazardous/bleach.png"),
            assets.load("trashes/hazardous/electronic.png"),
        ],
    });

    commands.trigger(SpawnTrashEvent);
}
