use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::util::sprite::get_bound;

/// Only work on center achor sprites.
#[derive(Component)]
#[require(Sprite, Transform)]
pub enum Align {
    Left,
    Right,
    Top,
    Bottom,
}

pub fn align_sync(
    mut resized: MessageReader<WindowResized>,
    assets: Res<Assets<Image>>,
    window: Query<&Window>,
    mut entities: Query<(&Sprite, &mut Transform, Ref<Align>)>,
) {
    let Ok(window) = window.single() else {
        return;
    };

    let resized = resized.read().last().is_some();

    for (sprite, mut transform, align) in &mut entities {
        if resized || align.is_changed() {
            apply_alignment(&assets, window, sprite, &mut transform, &align);
        }
    }
}

fn apply_alignment(
    assets: &Assets<Image>,
    window: &Window,
    sprite: &Sprite,
    transform: &mut Transform,
    align: &Align
) {
    if let Some(image) = assets.get(&sprite.image) {
        let bound = get_bound(&image, &transform);

        transform.translation = match align {
            Align::Left =>
                Vec3::new(
                    -(window.width() / 2.) + (bound.size().x / 2.),
                    transform.translation.y,
                    0.
                ),
            Align::Right =>
                Vec3::new(
                    (window.width() / 2.) - (bound.size().x / 2.),
                    transform.translation.y,
                    0.
                ),
            Align::Top =>
                Vec3::new(
                    transform.translation.x,
                (window.height() / 2.) - (bound.size().y / 2.),
                    0.
                ),
            Align::Bottom =>
                Vec3::new(
                    transform.translation.x,
                    -(window.height() / 2.) + (bound.size().y / 2.),
                    0.
                ),
        };
    }
}
