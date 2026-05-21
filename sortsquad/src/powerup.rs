use bevy::prelude::*;

use crate::util::random::random_from_list;

use crate::player::Player;
use crate::player::PlayerPowerCollector;

use crate::trashes::TrashKind;

use crate::items::Item;

use crate::column::Column;

// Powerup marker.
#[derive(Component)]
pub struct Powerup;

#[derive(Message)]
pub struct SpawnPowerupMessage;

#[derive(Event)]
pub struct DespawnPowerupEvent;

#[repr(u8)]
#[derive(Component)]
pub enum PowerupKind {
    Reveal,
}

pub fn powerup_spawner(
    mut msg: MessageReader<SpawnPowerupMessage>,
    mut commands: Commands,
    collector_player: Query<&PlayerPowerCollector>,
    players: Query<(Entity, &TrashKind), With<Player>>
) {
    if msg.read().last().is_none() || collector_player.iter().last().is_some() {
        return;
    }

    let collector_kind =
        random_from_list(&vec![
            TrashKind::General,
            TrashKind::Recycle,
            TrashKind::Organic,
            TrashKind::Hazardous
        ]);

    let Some(collector) = players
        .iter()
        .find_map(|(entity, pkind)| {
            if *pkind == collector_kind { Some(entity) } else { None }
        })
    else {
        error!("Can't find player to apply power collector to.");
        return;
    };

    commands.entity(collector).insert(PlayerPowerCollector);
}

pub fn powerup_despawn_observer(
    _trigger: On<DespawnPowerupEvent>,
    mut commands: Commands,
    collector_player: Query<Entity, (With<Player>, With<PlayerPowerCollector>)>,
) {
    let Ok(entity) = collector_player.single() else {
        error!("Only one player should have the PlayerPowerCollector component");
        return;
    };

    commands.entity(entity).remove::<PlayerPowerCollector>();
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
