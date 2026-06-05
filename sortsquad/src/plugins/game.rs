use bevy::prelude::*;

use crate::game::setup::desetup_game;
use crate::util::achor::achor_bottom_sync;

use crate::game::setup::setup_game;

use crate::state::GameState;
use crate::state::RoundState;

use crate::game::round::RoundCounter;
use crate::game::round::setup_round;
use crate::game::round::start_round;
use crate::game::round::round_increment;

use crate::game::timer::GameTimer;
use crate::game::timer::timer_update;

use crate::game::column::column_sync;

use crate::game::items::ItemsYPos;
use crate::game::items::spawn_items;
use crate::game::items::despawn_items;
use crate::game::items::items_gravity;

use crate::game::powerup::powerup_sync_text;
use crate::game::powerup::ActivePowerup;
use crate::game::powerup::powerup_despawn;

use crate::game::score::Score;
use crate::game::score::scoring;
use crate::game::score::sync_score_text;

use crate::game::player::players_move;
use crate::game::player::players_apply_collector_effect;
use crate::game::player::players_remove_collector_effect;
use crate::game::player::players_sync_control_label;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Score>()
            .init_resource::<ItemsYPos>()
            .init_resource::<ActivePowerup>()
            .init_resource::<RoundCounter>()
            .init_resource::<GameTimer>()
            .init_state::<RoundState>()
            .add_systems(OnEnter(GameState::GameSetup), setup_game)
            .add_systems(OnEnter(GameState::Ended), desetup_game)
            .add_systems(
                OnEnter(RoundState::RoundStarting),
                (spawn_items, start_round).chain()
            )
            .add_systems(
                OnEnter(RoundState::RoundEnding),
                (
                    round_increment,
                    powerup_despawn,
                    despawn_items,
                    setup_round
                ).chain()
            )
            .add_systems(
                Update,
                (
                    timer_update,
                    column_sync,
                    achor_bottom_sync,
                    items_gravity,
                    players_move,
                    players_sync_control_label,
                    players_apply_collector_effect,
                    players_remove_collector_effect,
                    scoring.run_if(resource_changed::<ItemsYPos>),
                    sync_score_text.run_if(resource_changed::<Score>),
                    powerup_sync_text.run_if(resource_changed::<ActivePowerup>)
                ).run_if(in_state(GameState::Playing))
            );
    }
}
