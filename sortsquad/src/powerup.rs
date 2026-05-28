use bevy::prelude::*;

use crate::player::Player;
use crate::player::PlayerPowerCollector;

use crate::items::Item;

use crate::column::Column;

// Powerup marker.
#[derive(Component)]
pub struct Powerup;

#[repr(u8)]
#[derive(Component, Debug, Clone, Copy)]
pub enum PowerupKind {
    NoPowerup,
    Reveal,
}

#[derive(Resource, Debug)]
pub struct ActivePowerup(pub PowerupKind); 

impl Default for ActivePowerup {
    fn default() -> Self {
        Self(PowerupKind::NoPowerup)
    }
}

#[derive(Bundle)]
pub struct PowerupBundle {
    pub item: Item,
    pub powerup: Powerup,
    pub col: Column,
    pub kind: PowerupKind,
    pub transform: Transform,
    pub sprite: Sprite,
}

pub fn powerup_despawn(
    mut commands: Commands,
    mut active: ResMut<ActivePowerup>,
    collector_player: Query<Entity, (With<Player>, With<PlayerPowerCollector>)>,
) {
    if let Ok(entity) = collector_player.single() {
        commands.entity(entity).remove::<PlayerPowerCollector>();
    };

    active.0 = PowerupKind::NoPowerup;
}
