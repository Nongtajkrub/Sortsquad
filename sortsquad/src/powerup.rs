use bevy::prelude::*;

use crate::player::Player;
use crate::player::PlayerPowerCollector;

use crate::items::Item;

use crate::column::Column;

// Powerup marker.
#[derive(Component)]
pub struct Powerup;

#[derive(Component)]
pub struct PowerupText;

#[repr(u8)]
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum PowerupKind {
    NoPowerup,
    AutoCorrect,
    DoublePoint,
    Highlight,
    SlowDown,
}

impl PowerupKind {
    pub const ALL: [Self; 4] = [
        Self::AutoCorrect,
        Self::DoublePoint,
        Self::Highlight,
        Self::SlowDown,
    ];

    pub fn to_text(&self) -> &'static str {
        use PowerupKind::*;

        match self {
            NoPowerup => "",
            AutoCorrect => "Auto Correct!",
            DoublePoint => "Double Point!",
            Highlight => "Highlight!",
            SlowDown => "Slow Down",
        }
    }
}

#[derive(Resource, Debug, PartialEq)]
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

pub fn powerup_sync_text(
    mut text: Single<&mut Text, With<PowerupText>>,
    active: Res<ActivePowerup>
) {
    text.0 = format!("{}", active.0.to_text());
}
