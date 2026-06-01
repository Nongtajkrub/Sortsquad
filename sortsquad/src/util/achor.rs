use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::configs::VIEW_PORT_WIDTH;

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct SpriteAchorBottom;

pub fn achor_bottom_sync(
    window: Query<&Window, With<PrimaryWindow>>,
    mut entities: Query<(&Sprite, &mut Transform), With<SpriteAchorBottom>>
) {
    let Ok(window) = window.single() else {
        return;
    };

    let bottom_y = -((VIEW_PORT_WIDTH * (window.height() / window.width())) / 2.);

    for (sprite, mut transform) in &mut entities {
        if let Some(size) = sprite.custom_size {
            transform.translation.y = bottom_y + (size.y / 2.);
        }
    }
}
