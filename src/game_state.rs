use bevy::prelude::*;
use crate::interface::{GameState, StatDisplay, StatType};

pub struct GameStatePlugin;

#[derive(Resource)]
pub struct PlayerGameState {
    pub health: u32,
    pub points: u32,
    pub game_id: u32,
    pub milestone: u32,
    pub orbs: u32,
    pub level: u32,
    pub moonrocks: u32,
    pub cheddah: u32,
}

impl Default for PlayerGameState {
    fn default() -> Self {
        Self {
            health: 100,
            points: 0,
            game_id: 1,
            milestone: 0,
            orbs: 0,
            level: 1,
            moonrocks: 0,
            cheddah: 0,
        }
    }
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(Update, update_stats_display.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), cleanup_game);
    }
}

fn setup_game(mut commands: Commands) {
    info!("Setting up game state");
    
    // Initialize player game state resource
    commands.insert_resource(PlayerGameState::default());
}

fn update_stats_display(
    player_state: Option<Res<PlayerGameState>>,
    mut stats_query: Query<(&mut Text, &StatDisplay)>,
) {
    if let Some(state) = player_state {
        for (mut text, stat_display) in &mut stats_query {
            **text = match stat_display.stat_type {
                StatType::Health => format!("Health: {}", state.health),
                StatType::Points => format!("Points: {}", state.points),
                StatType::GameId => format!("Game ID: {}", state.game_id),
                StatType::Milestone => format!("Milestone: {}", state.milestone),
                StatType::Orbs => format!("Orbs: {}", state.orbs),
                StatType::Level => format!("Level: {}", state.level),
                StatType::Moonrocks => format!("Moonrocks: {}", state.moonrocks),
                StatType::Cheddah => format!("Cheddah: {}", state.cheddah),
            };
        }
    }
}

fn cleanup_game(mut commands: Commands) {
    info!("Cleaning up game state");
    
    // Remove player game state resource
    commands.remove_resource::<PlayerGameState>();
}