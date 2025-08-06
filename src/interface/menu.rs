use bevy::prelude::*;
use super::{GameState, MenuUI, StartButton};

pub fn setup_menu_ui(mut commands: Commands) {
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

pub fn handle_start_button(
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

pub fn cleanup_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuUI>>) {
    for entity in &menu_query {
        commands.entity(entity).despawn();
    }
}