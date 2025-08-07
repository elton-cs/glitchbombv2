use bevy::{asset::AssetMetaCheck, prelude::*};

mod interface;
mod game_state;

use interface::GameState;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // === CORE BEVY SETUP ===
        // Configure Bevy plugins and window settings
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Glitch Bomb".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );
        
        // Initialize game state system
        app.init_state::<GameState>();
        
        // === STARTUP SYSTEMS ===
        // Run once when the app starts, before any state transitions
        app.add_systems(
            Startup, 
            interface::camera::setup_camera
        );
        
        // === STATE ENTRY SYSTEMS ===
        // Run when entering each game state (Menu -> Playing -> GameWon/GameLost)
        
        // Menu state entry - initial state
        app.add_systems(
            OnEnter(GameState::Menu), 
            interface::menu::setup_menu_ui
        );
        
        // Playing state entry - setup UI first, then game logic
        app.add_systems(
            OnEnter(GameState::Playing), 
            (
                interface::playing::setup_playing_ui, 
                game_state::systems::setup_game
            ).chain()
        );

        // Level complete state entry - setup transition screen
        app.add_systems(
            OnEnter(GameState::LevelComplete), 
            interface::level_complete::setup_level_complete_ui
        );
        
        // Marketplace state entry - setup marketplace UI
        app.add_systems(
            OnEnter(GameState::Marketplace), 
            interface::marketplace::setup_marketplace_ui
        );
        
        // Game end states - setup appropriate end screen UI
        app.add_systems(
            OnEnter(GameState::GameWon), 
            interface::game_end::setup_game_won_ui
        );
        
        app.add_systems(
            OnEnter(GameState::GameLost), 
            interface::game_end::setup_game_lost_ui
        );
        
        // === UPDATE SYSTEMS ===
        // Run every frame while in specific states
        
        // Menu update - handle start button clicks
        app.add_systems(
            Update, 
            interface::menu::handle_start_button
                .run_if(in_state(GameState::Menu))
        );
        
        // Playing update - chain all systems with quit handler last to avoid resource conflicts
        app.add_systems(
            Update, 
            (
                interface::playing::handle_pull_orb_button,
                game_state::systems::update_stats_display, 
                game_state::systems::check_win_loss_conditions,
                interface::playing::handle_quit_button
            ).chain().run_if(in_state(GameState::Playing))
        );

        // Level complete update - handle go to market button
        app.add_systems(
            Update, 
            interface::level_complete::handle_go_to_market_button
                .run_if(in_state(GameState::LevelComplete))
        );
        
        // Marketplace update - handle purchases and next level button
        app.add_systems(
            Update, 
            (
                interface::marketplace::handle_buy_health_orb,
                interface::marketplace::handle_buy_point_orb,
                interface::marketplace::update_marketplace_stats,
                interface::marketplace::handle_next_level_button
            ).chain().run_if(in_state(GameState::Marketplace))
        );
        
        // Game end update - handle restart button for both win/loss states
        app.add_systems(
            Update, 
            interface::game_end::handle_restart_button
                .run_if(in_state(GameState::GameWon).or(in_state(GameState::GameLost)))
        );
        
        // === STATE EXIT SYSTEMS ===
        // Run when leaving each game state to clean up resources
        
        app.add_systems(
            OnExit(GameState::Menu), 
            interface::menu::cleanup_menu
        );
        
        // Playing state exit - cleanup UI only (preserve game state for level transition)
        app.add_systems(
            OnExit(GameState::Playing), 
            interface::playing::cleanup_playing
        );
        
        // Game end states cleanup - also cleanup game state resource
        app.add_systems(
            OnExit(GameState::GameWon), 
            (
                interface::game_end::cleanup_game_end,
                game_state::systems::cleanup_game
            ).chain()
        );
        
        app.add_systems(
            OnExit(GameState::GameLost), 
            (
                interface::game_end::cleanup_game_end,
                game_state::systems::cleanup_game
            ).chain()
        );

        // Level complete state cleanup
        app.add_systems(
            OnExit(GameState::LevelComplete), 
            interface::level_complete::cleanup_level_complete
        );
        
        // Marketplace state cleanup
        app.add_systems(
            OnExit(GameState::Marketplace), 
            interface::marketplace::cleanup_marketplace
        );
    }
}
