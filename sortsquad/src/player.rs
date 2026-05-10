use bevy::prelude::*;
use bevy::ecs::query::QueryFilter;

use crate::util::sprite::get_bound;

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

/// Player marker
#[derive(Component)]
pub struct PlayerSlot {
    pub slot: i8,
}

impl PlayerSlot {
    pub fn new(slot: u8) -> Self {
        Self {
            slot: slot as i8,
        }
    }
}

#[derive(Event)]
pub struct PlayerSwappedEvent;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub slot: PlayerSlot,
    pub transform: Transform,
    pub sprite: Sprite,
}

fn move_player<F1, F2>(
    commands: &mut Commands,
    rightk: KeyCode,
    leftk: KeyCode,
    keyboard: &ButtonInput<KeyCode>,
    player: &mut Query<&mut PlayerSlot, F1>,
    others: &mut Query<&mut PlayerSlot, F2>
)
where 
    F1: QueryFilter,
    F2: QueryFilter,
{
    let mut pslot = player.single_mut().expect("One player per trash type only.");
    let direction: i8;

    if keyboard.just_pressed(rightk) {
        direction = 1;
    } else if keyboard.just_pressed(leftk) {
        direction = -1;
    } else {
        return;
    }

    for mut slot in others.iter_mut() {
        // If the entity slot is next to the player in the right direction.
        if slot.slot == (pslot.slot + direction) {
            std::mem::swap(&mut pslot.slot, &mut slot.slot);
            commands.trigger(PlayerSwappedEvent);
            break;
        }
    }
}

/// System for moving the general bin player.
pub fn move_general_player(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>, 
    mut player: Query<&mut PlayerSlot, With<GeneralPlayer>>,
    mut others: Query<&mut PlayerSlot, Without<GeneralPlayer>>
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
    mut player: Query<&mut PlayerSlot, With<RecyclePlayer>>,
    mut others: Query<&mut PlayerSlot, Without<RecyclePlayer>>
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
    mut player: Query<&mut PlayerSlot, With<RecyclePlayer>>,
    mut others: Query<&mut PlayerSlot, Without<RecyclePlayer>>
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
    mut player: Query<&mut PlayerSlot, With<HazardousPlayer>>,
    mut others: Query<&mut PlayerSlot, Without<HazardousPlayer>>
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

pub fn sync_player_slot(
    _trigger: On<PlayerSwappedEvent>,
    assets: Res<Assets<Image>>,
    window: Query<&Window>,
    mut players: Query<(&PlayerSlot, &mut Transform, &mut Sprite)>
) {
    let window = window.single().expect("No window entity.");

    let sprite_w = window.width() / 4.;
    let left_edge = -(window.width() / 2.);
    let bottom_edge = -(window.height() / 2.);

    for (slot, mut transform, sprite) in &mut players {
        if let Some(image) = assets.get(&sprite.image) {
            transform.scale = Vec3::splat(sprite_w / image.size_f32().x);

            let bound = get_bound(&image, &transform);

            transform.translation =
                Vec3::new(
                    left_edge + ((sprite_w * slot.slot as f32) + (sprite_w / 2.)),
                    (bound.size().y / 2.) + bottom_edge,
                    0.
                );
        }
    }
}
