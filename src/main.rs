use bevy::prelude::*;
use glitchbombv2::GamePlugin;

fn main() -> AppExit {
    App::new().add_plugins(GamePlugin).run()
}
