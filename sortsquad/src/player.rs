use bevy::prelude::*;
use bevy::ecs::query::QueryFilter;

use crate::util::column::Column;
use crate::util::column::ColumnResyncEvent;

/// Player marker
#[derive(Component)]
pub struct Player;

/// Player marker
#[derive(Component)]
pub struct GeneralPlayer;

/// Player marker
#[derive(Component)]
pub struct RecyclePlayer;

/// Player marker
#[derive(Component)]
pub struct OrganicPlayer;

/// Player marker
#[derive(Component)]
pub struct HazardousPlayer;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub col: Column,
    pub transform: Transform,
    pub sprite: Sprite,
}

fn move_player<F1, F2>(
    commands: &mut Commands,
    rightk: KeyCode,
    leftk: KeyCode,
    keyboard: &ButtonInput<KeyCode>,
    player: &mut Query<&mut Column, F1>,
    others: &mut Query<&mut Column, F2>
)
where 
    F1: QueryFilter,
    F2: QueryFilter,
{
    let mut pcol = player.single_mut().expect("One player per trash type only.");
    let direction: i32;

    if keyboard.just_pressed(rightk) {
        direction = 1;
    } else if keyboard.just_pressed(leftk) {
        direction = -1;
    } else {
        return;
    }

    for mut col in others.iter_mut() {
        // If the entity slot is next to the player in the right direction.
        if col.0 as i32 == (pcol.0 as i32 + direction) {
            std::mem::swap(&mut pcol.0, &mut col.0);
            commands.trigger(ColumnResyncEvent);
            break;
        }
    }
}

/// System for moving the general bin player.
pub fn move_general_player(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>, 
    mut player: Query<&mut Column, With<GeneralPlayer>>,
    mut others: Query<&mut Column, (Without<GeneralPlayer>, With<Player>)>
) {
    move_player(
        &mut commands,
        KeyCode::KeyD,
        KeyCode::KeyA,
        &keyboard,
        &mut player,
        &mut others
    );
}

/// System for moving the recycle bin player.
pub fn move_recycle_player(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>, 
    mut player: Query<&mut Column, With<RecyclePlayer>>,
    mut others: Query<&mut Column, (Without<RecyclePlayer>, With<Player>)>
) {
    move_player(
        &mut commands,
        KeyCode::KeyH,
        KeyCode::KeyG,
        &keyboard,
        &mut player,
        &mut others
    );
}

/// System for moving the organic bin player.
pub fn move_organic_player(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>, 
    mut player: Query<&mut Column, With<OrganicPlayer>>,
    mut others: Query<&mut Column, (Without<OrganicPlayer>, With<Player>)>
) {
    move_player(
        &mut commands,
        KeyCode::ArrowRight,
        KeyCode::ArrowLeft,
        &keyboard,
        &mut player,
        &mut others
    );
}

/// System for moving the recycle bin player.
pub fn move_hazardous_player(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>, 
    mut player: Query<&mut Column, With<HazardousPlayer>>,
    mut others: Query<&mut Column, (Without<HazardousPlayer>, With<Player>)>
) {
    move_player(
        &mut commands,
        KeyCode::BracketRight,
        KeyCode::BracketLeft,
        &keyboard,
        &mut player,
        &mut others
    );
}
