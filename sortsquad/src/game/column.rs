use bevy::prelude::*;

use crate::setup::VIEW_PORT_WIDTH;

const COLUMN_N: u32 = 4;

#[derive(Component)]
#[require(Transform, Sprite)]
pub struct Column {
    n: u32,

    /// Allow for custom size for sprite rather than the best fit size.
    pub size_factor: f32,
}

impl Column {
    pub fn new(n: u32) -> Self {
        assert!(n < COLUMN_N);

        Self {
            n: n,
            size_factor: 1.,
        }
    }

    /// Allow for custom size for sprite rather than the best fit size.
    pub fn with_size_factor(n: u32, size_factor: f32) -> Self {
        assert!(n < COLUMN_N);

        Self {
            n,
            size_factor,
        } 
    }

    pub fn get(&self) -> u32 {
        self.n
    }

    pub fn set(&mut self, c: u32) {
        assert!(c < COLUMN_N);
        self.n = c;
    }
}

/// Synce any entity(sprite) that have a Column component to the correct X position and scale.
pub fn column_sync(
    mut entities: Query<(&Column, &mut Transform, &mut Sprite)>
) {
    let sprite_w = VIEW_PORT_WIDTH / COLUMN_N as f32;
    let left_edge = -(VIEW_PORT_WIDTH / 2.);

    for (col, mut transform, mut sprite) in &mut entities {
        let size = sprite_w * col.size_factor;
        sprite.custom_size = Some(Vec2::new(size, size));

        transform.translation.x =
            left_edge + ((sprite_w * col.n as f32) + (sprite_w / 2.));
    }
}
