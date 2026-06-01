use bevy::prelude::*;
use bevy::camera::ScalingMode;

use crate::configs::*;

pub fn setup_main(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedHorizontal { 
                viewport_width: VIEW_PORT_WIDTH 
            }, 
            ..OrthographicProjection::default_2d()
        })
    ));
}
