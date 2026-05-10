use bevy::prelude::*;

#[inline]
pub fn get_bound(image: &Image, transform: &Transform) -> Rect {
    Rect::from_center_size(
        transform.translation.truncate(),
        image.size_f32() * transform.scale.truncate()
    )
}
