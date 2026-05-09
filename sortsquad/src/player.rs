use bevy::prelude::*;
use bevy::ecs::query::QueryFilter;

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

#[derive(Bundle)]
pub struct PlayerBundle {
    pub slot: PlayerSlot,
    pub transform: Transform,
    pub sprite: Sprite,
}

fn move_player<F1, F2>(
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
    let mut pslot = player.single_mut().unwrap();
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
            println!("Swap: {}, with {}", pslot.slot, slot.slot);
            std::mem::swap(&mut pslot.slot, &mut slot.slot);
            break;
        }
    }
}

/// System for moving the general bin player.
pub fn move_general_player(
    keyboard: Res<ButtonInput<KeyCode>>, 
    mut player: Query<&mut PlayerSlot, With<GeneralPlayer>>,
    mut others: Query<&mut PlayerSlot, Without<GeneralPlayer>>
) {
    move_player(
        KeyCode::KeyD,
        KeyCode::KeyA,
        &keyboard,
        &mut player,
        &mut others
    );
}

/// System for moving the recycle bin player.
pub fn move_recycle_player(
    keyboard: Res<ButtonInput<KeyCode>>, 
    mut player: Query<&mut PlayerSlot, With<RecyclePlayer>>,
    mut others: Query<&mut PlayerSlot, Without<RecyclePlayer>>
) {
    move_player(
        KeyCode::KeyG,
        KeyCode::KeyH,
        &keyboard,
        &mut player,
        &mut others
    );
}

/// System for moving the organic bin player.
pub fn move_organic_player(
    keyboard: Res<ButtonInput<KeyCode>>, 
    mut player: Query<&mut PlayerSlot, With<RecyclePlayer>>,
    mut others: Query<&mut PlayerSlot, Without<RecyclePlayer>>
) {
    move_player(
        KeyCode::ArrowRight,
        KeyCode::ArrowLeft,
        &keyboard,
        &mut player,
        &mut others
    );
}

/// System for moving the recycle bin player.
pub fn move_hazardous_player(
    keyboard: Res<ButtonInput<KeyCode>>, 
    mut player: Query<&mut PlayerSlot, With<HazardousPlayer>>,
    mut others: Query<&mut PlayerSlot, Without<HazardousPlayer>>
) {
    move_player(
        KeyCode::BracketRight,
        KeyCode::BracketLeft,
        &keyboard,
        &mut player,
        &mut others
    );
}
