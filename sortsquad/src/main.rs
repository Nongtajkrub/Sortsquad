use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, controllable_movement)
        .run();
}

#[derive(Component)]
struct Controllable;

fn setup(mut commands: Commands, asset: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(asset.load("bins/general/static.png")),
        Transform::from_xyz(0., 0., 0.),
        Controllable,
    ));
}

fn controllable_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut sprites: Query<&mut Transform, With<Controllable>>
) {
    for mut transform in &mut sprites {
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyW) {
            direction.y += 1.;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction.y -= 1.;
        }
        
        if keyboard.pressed(KeyCode::KeyD) {
            direction.x += 1.;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            direction.x -= 1.;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        transform.translation += direction * 400. * time.delta_secs();
    }
}
