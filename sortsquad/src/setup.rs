use bevy::prelude::*;

use crate::player::PlayerSlot;
use crate::player::PlayerBundle;
use crate::player::GeneralPlayer;
use crate::player::OrganicPlayer;
use crate::player::RecyclePlayer;
use crate::player::HazardousPlayer;

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        GeneralPlayer,
        PlayerBundle {
            slot: PlayerSlot::new(1),
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::from_image(assets.load("bins/general/static.png")),
        }
    ));

    commands.spawn((
        RecyclePlayer,
        PlayerBundle {
            slot: PlayerSlot::new(2),
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::from_image(assets.load("bins/general/static.png")),
        }
    ));

    commands.spawn((
        OrganicPlayer,
        PlayerBundle {
            slot: PlayerSlot::new(3),
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::from_image(assets.load("bins/general/static.png")),
        }
    ));

    commands.spawn((
        HazardousPlayer,
        PlayerBundle {
            slot: PlayerSlot::new(4),
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite::from_image(assets.load("bins/general/static.png")),
        }
    ));
}
