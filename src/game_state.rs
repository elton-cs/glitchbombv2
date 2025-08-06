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

#[allow(dead_code)]
impl PlayerGameState {
    // Getter methods
    pub fn health(&self) -> u32 { self.health }
    pub fn points(&self) -> u32 { self.points }
    pub fn game_id(&self) -> u32 { self.game_id }
    pub fn milestone(&self) -> u32 { self.milestone }
    pub fn orbs(&self) -> u32 { self.orbs }
    pub fn level(&self) -> u32 { self.level }
    pub fn moonrocks(&self) -> u32 { self.moonrocks }
    pub fn cheddah(&self) -> u32 { self.cheddah }

    // Setter methods
    pub fn set_health(&mut self, value: u32) { self.health = value; }
    pub fn set_points(&mut self, value: u32) { self.points = value; }
    pub fn set_game_id(&mut self, value: u32) { self.game_id = value; }
    pub fn set_milestone(&mut self, value: u32) { self.milestone = value; }
    pub fn set_orbs(&mut self, value: u32) { self.orbs = value; }
    pub fn set_level(&mut self, value: u32) { self.level = value; }
    pub fn set_moonrocks(&mut self, value: u32) { self.moonrocks = value; }
    pub fn set_cheddah(&mut self, value: u32) { self.cheddah = value; }

    // Increment methods
    pub fn add_health(&mut self, amount: u32) { self.health += amount; }
    pub fn add_points(&mut self, amount: u32) { self.points += amount; }
    pub fn add_orbs(&mut self, amount: u32) { self.orbs += amount; }
    pub fn add_moonrocks(&mut self, amount: u32) { self.moonrocks += amount; }
    pub fn add_cheddah(&mut self, amount: u32) { self.cheddah += amount; }
    
    pub fn increase_milestone(&mut self) { self.milestone += 1; }
    pub fn level_up(&mut self) { self.level += 1; }

    // Decrement methods (with bounds checking)
    pub fn subtract_health(&mut self, amount: u32) { 
        self.health = self.health.saturating_sub(amount); 
    }
    pub fn subtract_points(&mut self, amount: u32) { 
        self.points = self.points.saturating_sub(amount); 
    }
    pub fn subtract_orbs(&mut self, amount: u32) { 
        self.orbs = self.orbs.saturating_sub(amount); 
    }
    pub fn subtract_moonrocks(&mut self, amount: u32) { 
        self.moonrocks = self.moonrocks.saturating_sub(amount); 
    }
    pub fn subtract_cheddah(&mut self, amount: u32) { 
        self.cheddah = self.cheddah.saturating_sub(amount); 
    }

    // Utility methods
    pub fn is_dead(&self) -> bool { self.health == 0 }
    pub fn has_orbs(&self, count: u32) -> bool { self.orbs >= count }
    pub fn has_moonrocks(&self, count: u32) -> bool { self.moonrocks >= count }
    pub fn has_cheddah(&self, count: u32) -> bool { self.cheddah >= count }
    
    pub fn reset_to_defaults(&mut self) {
        *self = Self::default();
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