use bevy::prelude::States;

/// The game's gameplay states.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameStates {
    #[default]
    Playing,
    Selection,
    Paused,
    GameOver,
    Victory,
}

/// The game's states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AppStates {
    #[default]
    Splash,
    LoadingAssets,
    MainMenu,
    CharacterSelection,
    InitializeRun,
    Credits,
    Game,
    GameOver,
    Victory,
}
