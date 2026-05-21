use bevy::prelude::*;

use crate::items::ResetItemsEvent;

use crate::powerup::Powerup;
use crate::powerup::SpawnPowerupMessage;
use crate::powerup::DespawnPowerupEvent;

#[derive(Resource)]
pub struct RoundCounter(pub u32);

impl Default for RoundCounter {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Event)]
pub struct RoundIncrementEvent;

pub fn round_increment_observer(
    _trigger: On<RoundIncrementEvent>,
    mut commands: Commands,
    mut round: ResMut<RoundCounter>,
    mut msg: MessageWriter<SpawnPowerupMessage>,
    powerup: Query<&Powerup>,
) {
    round.0 += 1;

    commands.trigger(ResetItemsEvent);

    // Spawn power up every specific round if it does not exist.
    if round.0 != 0 && round.0 % 2 == 0 && powerup.iter().last().is_none() {
        msg.write(SpawnPowerupMessage);
    }

    // Despawn powerup when round end if powerup exist.
    if powerup.iter().last().is_some() {
        commands.trigger(DespawnPowerupEvent);
    }
}
