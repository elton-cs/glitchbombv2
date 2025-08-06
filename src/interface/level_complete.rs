use bevy::prelude::*;
use super::{GameState, LevelCompleteUI, NextLevelButton};
use crate::game_state::PlayerGameState;

pub fn setup_level_complete_ui(
    mut commands: Commands,
    player_state: Option<Res<PlayerGameState>>,
) {
    let (current_level, points_achieved, next_level) = if let Some(state) = player_state {
        (state.level(), state.points(), state.level() + 1)
    } else {
        (1, 0, 2)
    };

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
        LevelCompleteUI,
    ))
    .with_children(|parent| {
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
        ))
        .with_children(|content_parent| {
            content_parent.spawn((
                Text::new(format!("LEVEL {} COMPLETE!", current_level)),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.8, 0.2)),
            ));

            content_parent.spawn((
                Text::new(format!("Points Achieved: {}", points_achieved)),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            content_parent.spawn((
                Text::new(format!("Advancing to Level {}", next_level)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
        });

        parent.spawn((
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(70.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
            BorderColor(Color::srgb(0.4, 0.8, 0.4)),
            NextLevelButton,
        ))
        .with_children(|button_parent| {
            button_parent.spawn((
                Text::new("NEXT LEVEL"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn handle_next_level_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<NextLevelButton>),
    >,
    mut player_state: Option<ResMut<PlayerGameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut button_pressed = false;
    
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(Color::srgb(0.1, 0.4, 0.1));
                *border_color = BorderColor(Color::srgb(0.2, 0.6, 0.2));
                button_pressed = true;
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::srgb(0.3, 0.7, 0.3));
                *border_color = BorderColor(Color::srgb(0.5, 0.9, 0.5));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::srgb(0.2, 0.6, 0.2));
                *border_color = BorderColor(Color::srgb(0.4, 0.8, 0.4));
            }
        }
    }
    
    if button_pressed {
        if let Some(ref mut state) = player_state {
            state.advance_to_next_level();
            info!("Advanced to level {}", state.level());
        }
        next_state.set(GameState::Playing);
    }
}

pub fn cleanup_level_complete(mut commands: Commands, level_complete_query: Query<Entity, With<LevelCompleteUI>>) {
    for entity in &level_complete_query {
        commands.entity(entity).despawn();
    }
}