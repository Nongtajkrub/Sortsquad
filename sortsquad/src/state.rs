use bevy::prelude::*;

#[derive(States, Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum GameState {
    #[default]
    AssetsLoading,
    Menu,
    Starting,
    Playing,
    Ended,
}

#[derive(States, Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum RoundState {
    #[default]
    GameSetup,
    RoundStarting,
    InRound,
    RoundEnding,
    NoMoreRound,
}
