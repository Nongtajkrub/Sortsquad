use bevy::prelude::*;

#[derive(Event)]
pub struct ColumnResyncEvent;

#[derive(Component)]
#[require(Transform, Sprite)]
pub struct Column {
    pub n: u32,

    /// Allow for custom size for sprite rather than the best fit size.
    pub size_factor: f32,
}

impl Column {
    pub fn new(n: u32) -> Self {
        Self {
            n: n,
            size_factor: 1.,
        }
    }

    /// Allow for custom size for sprite rather than the best fit size.
    pub fn with_size_factor(n: u32, size_factor: f32) -> Self {
        Self {
            n,
            size_factor,
        } 
    }
}

/// Synce any entity(sprite) that have a Column component to the correct X position and scale.
pub fn column_sync(
    _trigger: On<ColumnResyncEvent>,
    assets: Res<Assets<Image>>,
    window: Query<&Window>,
    mut entities: Query<(&Column, &mut Transform, &mut Sprite)>
) {
    let window = window.single().expect("No window entity.");

    let sprite_w = window.width() / 4.;
    let left_edge = -(window.width() / 2.);

    for (col, mut transform, sprite) in &mut entities {
        if let Some(image) = assets.get(&sprite.image) {
            transform.scale =
                Vec3::splat((sprite_w / image.size_f32().x) * col.size_factor);

            transform.translation =
                Vec3::new(
                    left_edge + ((sprite_w * col.n as f32) + (sprite_w / 2.)),
                    transform.translation.y,
                    0.
                );
        }
    }
}
