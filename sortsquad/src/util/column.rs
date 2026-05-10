use bevy::prelude::*;

#[derive(Event)]
pub struct ColumnResyncEvent;

#[derive(Component)]
#[require(Transform, Sprite)]
pub struct Column(pub u32);

impl Column {
    pub fn new(n: u32) -> Self {
        Column(n)
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

    for (column, mut transform, sprite) in &mut entities {
        if let Some(image) = assets.get(&sprite.image) {
            transform.scale = Vec3::splat(sprite_w / image.size_f32().x);
            transform.translation =
                Vec3::new(
                    left_edge + ((sprite_w * column.0 as f32) + (sprite_w / 2.)),
                    transform.translation.y,
                    0.
                );
        }
    }
}
