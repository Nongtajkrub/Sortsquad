use bevy::prelude::*;

use crate::player::PlayerControl;
use crate::player::PlayerBundle;
use crate::player::Player;

use crate::trash::SpawnTrashEvent;
use crate::trash::TrashImages;
use crate::trash::TrashKind;
use crate::util::column::Column;

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(
        PlayerBundle {
            player: Player,
            kind: TrashKind::General,
            col: Column::new(0),
            control: PlayerControl {
                left: KeyCode::KeyA,
                right: KeyCode::KeyD,
            },
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::from_image(assets.load("bins/general/static.png")),
        }
    );

    commands.spawn(
        PlayerBundle {
            player: Player,
            kind: TrashKind::Recycle,
            col: Column::new(1),
            control: PlayerControl {
                left: KeyCode::KeyG,
                right: KeyCode::KeyH
            },
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::from_image(assets.load("bins/recyclable/static.png")),
        }
    );

    commands.spawn(
        PlayerBundle {
            player: Player,
            kind: TrashKind::Organic,
            col: Column::new(2),
            control: PlayerControl {
                left: KeyCode::ArrowLeft,
                right: KeyCode::ArrowRight
            },
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::from_image(assets.load("bins/organic/static.png")),
        }
    );

    commands.spawn(
        PlayerBundle {
            player: Player,
            kind: TrashKind::Hazardous,
            col: Column::new(3),
            control: PlayerControl {
                left: KeyCode::BracketLeft,
                right: KeyCode::BracketRight
            },
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::from_image(assets.load("bins/hazardous/static.png")),
        }
    );

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
