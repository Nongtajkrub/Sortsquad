use bevy::prelude::*;

#[derive(States, Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum GameState {
    #[default]
    AssetsLoading,
    Starting,
    Playing,
}

#[derive(States, Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum RoundState {
    #[default]
    GameSetup,
    RoundStarting,
    InRound,
    RoundEnding,
}
