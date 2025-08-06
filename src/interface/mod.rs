use bevy::prelude::*;

pub mod states;
pub mod components;
pub mod camera;
pub mod menu;
pub mod playing;
pub mod game_end;

pub use states::*;
pub use components::*;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Startup, camera::setup_camera)
            .add_systems(OnEnter(GameState::Menu), menu::setup_menu_ui)
            .add_systems(Update, menu::handle_start_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), menu::cleanup_menu)
            .add_systems(OnEnter(GameState::Playing), playing::setup_playing_ui)
            .add_systems(Update, (playing::handle_quit_button, playing::handle_pull_orb_button).run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), playing::cleanup_playing)
            .add_systems(OnEnter(GameState::GameWon), game_end::setup_game_won_ui)
            .add_systems(Update, game_end::handle_restart_button.run_if(in_state(GameState::GameWon).or(in_state(GameState::GameLost))))
            .add_systems(OnExit(GameState::GameWon), game_end::cleanup_game_end)
            .add_systems(OnEnter(GameState::GameLost), game_end::setup_game_lost_ui)
            .add_systems(OnExit(GameState::GameLost), game_end::cleanup_game_end);
    }
}