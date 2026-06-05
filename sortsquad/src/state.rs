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
    PreTitlescreen,
    Titlescreen,
    Cutscene,
    CutsceneEnded,
}

#[derive(States, Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum RoundState {
    #[default]
    PreRound,
    RoundStarting,
    InRound,
    RoundEnding,
    NoMoreRound,
}
