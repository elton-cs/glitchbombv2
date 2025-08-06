use bevy::{asset::AssetMetaCheck, prelude::*};

mod interface;
mod game_state;

use interface::GameState;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
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
        
        app.init_state::<GameState>();
        
        app.add_systems(
            Startup, 
            interface::camera::setup_camera
        );
        
        app.add_systems(
            OnEnter(GameState::Menu), 
            interface::menu::setup_menu_ui
        );
        
        app.add_systems(
            OnEnter(GameState::Playing), 
            (
                interface::playing::setup_playing_ui, 
                game_state::systems::setup_game
            ).chain()
        );
        
        app.add_systems(
            OnEnter(GameState::GameWon), 
            interface::game_end::setup_game_won_ui
        );
        
        app.add_systems(
            OnEnter(GameState::GameLost), 
            interface::game_end::setup_game_lost_ui
        );
        
        app.add_systems(
            Update, 
            (
                interface::menu::handle_start_button
            ).run_if(in_state(GameState::Menu))
        );
        
        app.add_systems(
            Update, 
            (
                interface::playing::handle_quit_button, 
                interface::playing::handle_pull_orb_button,
                game_state::systems::update_stats_display, 
                game_state::systems::check_win_loss_conditions
            ).chain().run_if(in_state(GameState::Playing))
        );
        
        app.add_systems(
            Update, 
            interface::game_end::handle_restart_button
                .run_if(in_state(GameState::GameWon).or(in_state(GameState::GameLost)))
        );
        
        app.add_systems(
            OnExit(GameState::Menu), 
            interface::menu::cleanup_menu
        );
        
        app.add_systems(
            OnExit(GameState::Playing), 
            (
                interface::playing::cleanup_playing, 
                game_state::systems::cleanup_game
            ).chain()
        );
        
        app.add_systems(
            OnExit(GameState::GameWon), 
            interface::game_end::cleanup_game_end
        );
        
        app.add_systems(
            OnExit(GameState::GameLost), 
            interface::game_end::cleanup_game_end
        );
    }
}
