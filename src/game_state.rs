use bevy::prelude::*;
use crate::interface::{GameState, HealthText, PointsText, GameIdText, MilestoneText, OrbsText, LevelText, MoonrocksText, CheddahText};

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
    mut health_query: Query<&mut Text, (With<HealthText>, Without<PointsText>, Without<GameIdText>, Without<MilestoneText>, Without<OrbsText>, Without<LevelText>, Without<MoonrocksText>, Without<CheddahText>)>,
    mut points_query: Query<&mut Text, (With<PointsText>, Without<HealthText>, Without<GameIdText>, Without<MilestoneText>, Without<OrbsText>, Without<LevelText>, Without<MoonrocksText>, Without<CheddahText>)>,
    mut game_id_query: Query<&mut Text, (With<GameIdText>, Without<HealthText>, Without<PointsText>, Without<MilestoneText>, Without<OrbsText>, Without<LevelText>, Without<MoonrocksText>, Without<CheddahText>)>,
    mut milestone_query: Query<&mut Text, (With<MilestoneText>, Without<HealthText>, Without<PointsText>, Without<GameIdText>, Without<OrbsText>, Without<LevelText>, Without<MoonrocksText>, Without<CheddahText>)>,
    mut orbs_query: Query<&mut Text, (With<OrbsText>, Without<HealthText>, Without<PointsText>, Without<GameIdText>, Without<MilestoneText>, Without<LevelText>, Without<MoonrocksText>, Without<CheddahText>)>,
    mut level_query: Query<&mut Text, (With<LevelText>, Without<HealthText>, Without<PointsText>, Without<GameIdText>, Without<MilestoneText>, Without<OrbsText>, Without<MoonrocksText>, Without<CheddahText>)>,
    mut moonrocks_query: Query<&mut Text, (With<MoonrocksText>, Without<HealthText>, Without<PointsText>, Without<GameIdText>, Without<MilestoneText>, Without<OrbsText>, Without<LevelText>, Without<CheddahText>)>,
    mut cheddah_query: Query<&mut Text, (With<CheddahText>, Without<HealthText>, Without<PointsText>, Without<GameIdText>, Without<MilestoneText>, Without<OrbsText>, Without<LevelText>, Without<MoonrocksText>)>,
) {
    if let Some(state) = player_state {
        // Update health text
        if let Ok(mut text) = health_query.single_mut() {
            **text = format!("Health: {}", state.health);
        }
        
        // Update points text
        if let Ok(mut text) = points_query.single_mut() {
            **text = format!("Points: {}", state.points);
        }
        
        // Update game ID text
        if let Ok(mut text) = game_id_query.single_mut() {
            **text = format!("Game ID: {}", state.game_id);
        }
        
        // Update milestone text
        if let Ok(mut text) = milestone_query.single_mut() {
            **text = format!("Milestone: {}", state.milestone);
        }
        
        // Update orbs text
        if let Ok(mut text) = orbs_query.single_mut() {
            **text = format!("Orbs: {}", state.orbs);
        }
        
        // Update level text
        if let Ok(mut text) = level_query.single_mut() {
            **text = format!("Level: {}", state.level);
        }
        
        // Update moonrocks text
        if let Ok(mut text) = moonrocks_query.single_mut() {
            **text = format!("Moonrocks: {}", state.moonrocks);
        }
        
        // Update cheddah text
        if let Ok(mut text) = cheddah_query.single_mut() {
            **text = format!("Cheddah: {}", state.cheddah);
        }
    }
}

fn cleanup_game(mut commands: Commands) {
    info!("Cleaning up game state");
    
    // Remove player game state resource
    commands.remove_resource::<PlayerGameState>();
}