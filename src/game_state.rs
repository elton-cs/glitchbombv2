use bevy::prelude::*;
use crate::interface::{GameState, StatDisplay, StatType};
use rand::Rng;

pub struct GameStatePlugin;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Orb {
    Health,
    Point,
    Bomb,
}

#[derive(Resource)]
pub struct PlayerGameState {
    pub health: u32,
    pub points: u32,
    pub game_id: u32,
    pub milestone: u32,
    pub orbs: Vec<Orb>,
    pub level: u32,
    pub moonrocks: u32,
    pub cheddah: u32,
}

impl Default for PlayerGameState {
    fn default() -> Self {
        let mut orbs = Vec::new();
        
        // Add 5 of each orb type
        for _ in 0..5 {
            orbs.push(Orb::Health);
            orbs.push(Orb::Point);
            orbs.push(Orb::Bomb);
        }
        
        Self {
            health: 5,
            points: 0,
            game_id: 1,
            milestone: 15,
            orbs,
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
    pub fn orbs(&self) -> &Vec<Orb> { &self.orbs }
    pub fn level(&self) -> u32 { self.level }
    pub fn moonrocks(&self) -> u32 { self.moonrocks }
    pub fn cheddah(&self) -> u32 { self.cheddah }

    // Setter methods
    pub fn set_health(&mut self, value: u32) { self.health = value; }
    pub fn set_points(&mut self, value: u32) { self.points = value; }
    pub fn set_game_id(&mut self, value: u32) { self.game_id = value; }
    pub fn set_milestone(&mut self, value: u32) { self.milestone = value; }
    pub fn set_orbs(&mut self, orbs: Vec<Orb>) { self.orbs = orbs; }
    pub fn set_level(&mut self, value: u32) { self.level = value; }
    pub fn set_moonrocks(&mut self, value: u32) { self.moonrocks = value; }
    pub fn set_cheddah(&mut self, value: u32) { self.cheddah = value; }

    // Increment methods
    pub fn add_health(&mut self, amount: u32) { 
        self.health = (self.health + amount).min(5); 
    }
    pub fn add_points(&mut self, amount: u32) { self.points += amount; }
    pub fn add_orb(&mut self, orb: Orb) { self.orbs.push(orb); }
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
    pub fn remove_orb(&mut self, orb_type: Orb) -> bool {
        if let Some(pos) = self.orbs.iter().position(|&orb| orb == orb_type) {
            self.orbs.remove(pos);
            true
        } else {
            false
        }
    }
    pub fn subtract_moonrocks(&mut self, amount: u32) { 
        self.moonrocks = self.moonrocks.saturating_sub(amount); 
    }
    pub fn subtract_cheddah(&mut self, amount: u32) { 
        self.cheddah = self.cheddah.saturating_sub(amount); 
    }

    // Utility methods
    pub fn is_dead(&self) -> bool { self.health == 0 }
    pub fn is_at_max_health(&self) -> bool { self.health == 5 }
    pub fn has_orb(&self, orb_type: Orb) -> bool { 
        self.orbs.contains(&orb_type) 
    }
    pub fn orb_count(&self, orb_type: Orb) -> usize {
        self.orbs.iter().filter(|&&orb| orb == orb_type).count()
    }
    pub fn total_orb_count(&self) -> usize {
        self.orbs.len()
    }
    pub fn health_orb_count(&self) -> usize {
        self.orb_count(Orb::Health)
    }
    pub fn point_orb_count(&self) -> usize {
        self.orb_count(Orb::Point)
    }
    pub fn bomb_orb_count(&self) -> usize {
        self.orb_count(Orb::Bomb)
    }
    pub fn has_moonrocks(&self, count: u32) -> bool { self.moonrocks >= count }
    pub fn has_cheddah(&self, count: u32) -> bool { self.cheddah >= count }
    
    pub fn reset_to_defaults(&mut self) {
        *self = Self::default();
    }

    // Pull and consume a random orb
    pub fn pull_orb(&mut self) -> Option<Orb> {
        if self.orbs.is_empty() {
            return None;
        }

        // Generate random index
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.orbs.len());
        
        // Remove the orb at random index
        let orb = self.orbs.remove(random_index);
        
        // Apply orb effects
        match orb {
            Orb::Health => {
                if !self.is_at_max_health() {
                    self.add_health(1);
                    info!("Consumed Health orb: +1 health (now {})", self.health);
                } else {
                    info!("Consumed Health orb: no effect (health already at max)");
                }
            },
            Orb::Point => {
                self.add_points(5);
                info!("Consumed Point orb: +5 points");
            },
            Orb::Bomb => {
                self.subtract_health(2);
                info!("Consumed Bomb orb: -2 health");
            },
        }

        Some(orb)
    }
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(Update, (update_stats_display, check_win_loss_conditions).run_if(in_state(GameState::Playing)))
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

fn check_win_loss_conditions(
    player_state: Option<Res<PlayerGameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(state) = player_state {
        // Check win condition: points >= milestone
        if state.points >= state.milestone {
            info!("Player wins! Points: {} >= Milestone: {}", state.points, state.milestone);
            next_state.set(GameState::GameWon);
        }
        // Check loss condition: health <= 0
        else if state.health == 0 {
            info!("Player loses! Health reached zero.");
            next_state.set(GameState::GameLost);
        }
    }
}

fn cleanup_game(mut commands: Commands) {
    info!("Cleaning up game state");
    
    // Remove player game state resource
    commands.remove_resource::<PlayerGameState>();
}