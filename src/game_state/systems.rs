use bevy::prelude::*;
use crate::interface::{GameState, StatDisplay, StatType};
use super::PlayerGameState;

pub fn setup_game(mut commands: Commands) {
    info!("Setting up game state");
    
    commands.insert_resource(PlayerGameState::default());
}

pub fn update_stats_display(
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
                StatType::Orbs => format!("Orbs: H:{} P:{} B:{}", 
                    state.health_orb_count(), 
                    state.point_orb_count(), 
                    state.bomb_orb_count()
                ),
                StatType::Level => format!("Level: {}", state.level),
                StatType::Moonrocks => format!("Moonrocks: {}", state.moonrocks),
                StatType::Cheddah => format!("Cheddah: {}", state.cheddah),
            };
        }
    }
}

pub fn check_win_loss_conditions(
    player_state: Option<Res<PlayerGameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(state) = player_state {
        if state.points >= state.milestone {
            info!("Player wins! Points: {} >= Milestone: {}", state.points, state.milestone);
            next_state.set(GameState::GameWon);
        }
        else if state.health == 0 {
            info!("Player loses! Health reached zero.");
            next_state.set(GameState::GameLost);
        }
    }
}

pub fn cleanup_game(mut commands: Commands) {
    info!("Cleaning up game state");
    
    commands.remove_resource::<PlayerGameState>();
}