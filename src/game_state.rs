use bevy::prelude::*;
use crate::interface::{GameState, HealthText, PointsText, GameIdText, MilestoneText, OrbsText};

pub struct GameStatePlugin;

#[derive(Resource)]
pub struct PlayerGameState {
    pub health: u32,
    pub points: u32,
    pub game_id: u32,
    pub milestone: u32,
    pub orbs: u32,
}

impl Default for PlayerGameState {
    fn default() -> Self {
        Self {
            health: 100,
            points: 0,
            game_id: 1,
            milestone: 0,
            orbs: 0,
        }
    }
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(Update, (update_game, update_stats_display).run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), cleanup_game);
    }
}

#[derive(Component)]
struct GameEntity;

fn setup_game(mut commands: Commands) {
    info!("Setting up game state");
    
    // Initialize player game state resource
    commands.insert_resource(PlayerGameState::default());
    
    // Placeholder: spawn a simple game entity (not visible, just for demo)
    commands.spawn((
        Transform::default(),
        GameEntity,
    ));
}

fn update_game(player_state: Option<Res<PlayerGameState>>) {
    // Placeholder: game logic would go here
    // This runs every frame while in Playing state
    
    if let Some(_state) = player_state {
        // Access player state fields: _state.health, _state.points, _state.game_id, _state.milestone, _state.orbs
    }
}


fn update_stats_display(
    player_state: Option<Res<PlayerGameState>>,
    mut health_query: Query<&mut Text, (With<HealthText>, Without<PointsText>, Without<GameIdText>, Without<MilestoneText>, Without<OrbsText>)>,
    mut points_query: Query<&mut Text, (With<PointsText>, Without<HealthText>, Without<GameIdText>, Without<MilestoneText>, Without<OrbsText>)>,
    mut game_id_query: Query<&mut Text, (With<GameIdText>, Without<HealthText>, Without<PointsText>, Without<MilestoneText>, Without<OrbsText>)>,
    mut milestone_query: Query<&mut Text, (With<MilestoneText>, Without<HealthText>, Without<PointsText>, Without<GameIdText>, Without<OrbsText>)>,
    mut orbs_query: Query<&mut Text, (With<OrbsText>, Without<HealthText>, Without<PointsText>, Without<GameIdText>, Without<MilestoneText>)>,
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
    }
}

fn cleanup_game(mut commands: Commands, game_query: Query<Entity, With<GameEntity>>) {
    info!("Cleaning up game state");
    
    // Remove player game state resource
    commands.remove_resource::<PlayerGameState>();
    
    // Clean up game entities
    for entity in &game_query {
        commands.entity(entity).despawn();
    }
}