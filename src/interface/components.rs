use bevy::prelude::*;

#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct PlayingUI;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct PullOrbButton;

#[derive(Component)]
pub struct GameEndUI;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct LevelCompleteUI;

#[derive(Component)]
pub struct PullHistoryContainer;

#[derive(Component)]
pub struct PullHistoryOrb {
    pub _orb_type: crate::game_state::orb::Orb,
    pub position: usize,
    pub animation_timer: f32,
}