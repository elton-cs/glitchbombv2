use bevy::prelude::*;
use crate::interface::{GameState, StatDisplay, StatType};
use super::PlayerGameState;

pub fn setup_game(
    mut commands: Commands,
    player_state: Option<Res<PlayerGameState>>,
) {
    info!("Setting up game state");
    
    // Only create new PlayerGameState if one doesn't exist (first time playing)
    if player_state.is_none() {
        info!("Creating new PlayerGameState");
        commands.insert_resource(PlayerGameState::default());
    } else {
        info!("PlayerGameState already exists for level {}", player_state.unwrap().level());
    }
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
    mut player_state: Option<ResMut<PlayerGameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(ref mut state) = player_state {
        if state.points >= state.milestone {
            // Award cheddah for completing the level
            state.complete_level();
            
            if state.is_final_level() {
                info!("Player wins the game! Completed level {} with {} points", state.level, state.points);
                next_state.set(GameState::GameWon);
            } else {
                info!("Level {} completed! Going to level complete screen", state.level);
                next_state.set(GameState::LevelComplete);
            }
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