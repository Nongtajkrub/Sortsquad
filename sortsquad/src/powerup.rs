use bevy::prelude::*;

use crate::player::Player;
use crate::player::PlayerPowerCollector;

use crate::items::Item;

use crate::column::Column;

// Powerup marker.
#[derive(Component)]
pub struct Powerup;

#[repr(u8)]
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum PowerupKind {
    NoPowerup,
    AutoCorrect,
}

impl PowerupKind {
    pub const ALL: [Self; 1] = [
        Self::AutoCorrect,
    ];
}

#[derive(Resource, Debug, PartialEq)]
pub struct ActivePowerup(pub PowerupKind); 

impl Default for ActivePowerup {
    fn default() -> Self {
        Self(PowerupKind::NoPowerup)
    }
}

#[derive(Event)]
pub struct PowerupApplyAutoCorrectEvent;

#[derive(Bundle)]
pub struct PowerupBundle {
    pub item: Item,
    pub powerup: Powerup,
    pub col: Column,
    pub transform: Transform,
    pub sprite: Sprite,
}

pub fn powerup_despawn(
    mut commands: Commands,
    collector_player: Query<Entity, (With<Player>, With<PlayerPowerCollector>)>,
) {
    if let Ok(entity) = collector_player.single() {
        commands.entity(entity).remove::<PlayerPowerCollector>();
    };
}

pub fn powerup_apply(mut commands: Commands, active: Res<ActivePowerup>) {
    use PowerupKind::*;

    match active.0 {
        NoPowerup => (),
        AutoCorrect => commands.trigger(PowerupApplyAutoCorrectEvent),
    };
}
