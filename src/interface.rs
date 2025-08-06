use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
}

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Startup, setup_camera)
            .add_systems(OnEnter(GameState::Menu), setup_menu_ui)
            .add_systems(Update, handle_start_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu)
            .add_systems(OnEnter(GameState::Playing), setup_playing_ui)
            .add_systems(Update, handle_quit_button.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), cleanup_playing);
    }
}

#[derive(Component)]
struct MenuUI;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct PlayingUI;

#[derive(Component)]
struct QuitButton;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_menu_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        MenuUI,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("GLITCH BOMB"),
            TextFont {
                font_size: 64.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ));
        
        parent.spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            BorderColor(Color::srgb(0.4, 0.4, 0.4)),
            StartButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("START"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    });
}

fn handle_start_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<StartButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(Color::srgb(0.1, 0.1, 0.1));
                *border_color = BorderColor(Color::srgb(0.3, 0.3, 0.3));
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
                *border_color = BorderColor(Color::srgb(0.6, 0.6, 0.6));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
                *border_color = BorderColor(Color::srgb(0.4, 0.4, 0.4));
            }
        }
    }
}

fn setup_playing_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        PlayingUI,
    ))
    .with_children(|parent| {
        parent.spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            BorderColor(Color::srgb(0.4, 0.4, 0.4)),
            QuitButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("QUIT"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    });
}

fn handle_quit_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(Color::srgb(0.1, 0.1, 0.1));
                *border_color = BorderColor(Color::srgb(0.3, 0.3, 0.3));
                next_state.set(GameState::Menu);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
                *border_color = BorderColor(Color::srgb(0.6, 0.6, 0.6));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
                *border_color = BorderColor(Color::srgb(0.4, 0.4, 0.4));
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuUI>>) {
    for entity in &menu_query {
        commands.entity(entity).despawn();
    }
}

fn cleanup_playing(mut commands: Commands, playing_query: Query<Entity, With<PlayingUI>>) {
    for entity in &playing_query {
        commands.entity(entity).despawn();
    }
}