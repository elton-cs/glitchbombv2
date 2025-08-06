use bevy::prelude::*;
use super::{GameState, GameEndUI, RestartButton};

pub fn setup_game_won_ui(mut commands: Commands) {
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
        GameEndUI,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("YOU WIN!"),
            TextFont {
                font_size: 72.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 0.8, 0.0)),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ));
        
        parent.spawn((
            Text::new("Congratulations! You reached the milestone!"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(30.0)),
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
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            BorderColor(Color::srgb(0.4, 0.4, 0.4)),
            RestartButton,
        ))
        .with_children(|button_parent| {
            button_parent.spawn((
                Text::new("PLAY AGAIN"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn setup_game_lost_ui(mut commands: Commands) {
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
        GameEndUI,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("GAME OVER"),
            TextFont {
                font_size: 72.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.0, 0.0)),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ));
        
        parent.spawn((
            Text::new("Your health reached zero!"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(30.0)),
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
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            BorderColor(Color::srgb(0.4, 0.4, 0.4)),
            RestartButton,
        ))
        .with_children(|button_parent| {
            button_parent.spawn((
                Text::new("TRY AGAIN"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn handle_restart_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<RestartButton>),
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

pub fn cleanup_game_end(mut commands: Commands, game_end_query: Query<Entity, With<GameEndUI>>) {
    for entity in &game_end_query {
        commands.entity(entity).despawn();
    }
}