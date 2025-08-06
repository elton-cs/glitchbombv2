use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    GameWon,
    GameLost,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum StatType {
    Health,
    Points,
    GameId,
    Milestone,
    Orbs,
    Level,
    Moonrocks,
    Cheddah,
}

#[derive(Component)]
pub struct StatDisplay {
    pub stat_type: StatType,
}