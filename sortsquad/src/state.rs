use bevy::prelude::*;

#[derive(States, Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum GameState {
    #[default]
    AssetsLoading,
    MenuSetup,
    Menu,
    GameSetup,
    Playing,
    Ended,
}

#[derive(States, Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum MenuState {
    #[default]
    Main,
    Cutscene,
    CutsceneEnded,
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
