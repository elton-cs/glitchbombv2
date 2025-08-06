use bevy::prelude::*;
use crate::interface::GameState;

pub mod orb;
pub mod player;
pub mod systems;

pub use orb::*;
pub use player::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), systems::setup_game)
            .add_systems(Update, (systems::update_stats_display, systems::check_win_loss_conditions).run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), systems::cleanup_game);
    }
}