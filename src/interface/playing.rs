use bevy::prelude::*;
use super::{GameState, PlayingUI, QuitButton, PullOrbButton, StatDisplay, StatType, PullHistoryContainer, PullHistoryOrb};
use crate::game_state::orb::Orb;

pub fn setup_playing_ui(mut commands: Commands) {
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
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(5.0),
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            },
        ))
        .with_children(|stats_parent| {
            stats_parent.spawn((
                Text::new("Health: 5"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                StatDisplay { stat_type: StatType::Health },
            ));
            
            stats_parent.spawn((
                Text::new("Points: 0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                StatDisplay { stat_type: StatType::Points },
            ));
            
            stats_parent.spawn((
                Text::new("Game ID: 1"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                StatDisplay { stat_type: StatType::GameId },
            ));
            
            stats_parent.spawn((
                Text::new("Milestone: 15"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                StatDisplay { stat_type: StatType::Milestone },
            ));
            
            stats_parent.spawn((
                Text::new("Orbs: H:5 P:5 B:5"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                StatDisplay { stat_type: StatType::Orbs },
            ));
            
            stats_parent.spawn((
                Text::new("Level: 1"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                StatDisplay { stat_type: StatType::Level },
            ));
            
            stats_parent.spawn((
                Text::new("Moonrocks: 0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                StatDisplay { stat_type: StatType::Moonrocks },
            ));
            
            stats_parent.spawn((
                Text::new("Cheddah: 0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                StatDisplay { stat_type: StatType::Cheddah },
            ));
        });

        // Pull History Display
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ))
        .with_children(|history_parent| {
            history_parent.spawn((
                Text::new("Pull History:"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
            
            history_parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                },
                PullHistoryContainer,
            ));
        });

        parent.spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.4, 0.2)),
            BorderColor(Color::srgb(0.4, 0.6, 0.4)),
            PullOrbButton,
        ))
        .with_children(|button_parent| {
            button_parent.spawn((
                Text::new("PULL ORB"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });

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
        .with_children(|button_parent| {
            button_parent.spawn((
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

pub fn handle_quit_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut player_state: Option<ResMut<crate::game_state::PlayerGameState>>,
) {
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(Color::srgb(0.1, 0.1, 0.1));
                *border_color = BorderColor(Color::srgb(0.3, 0.3, 0.3));
                
                // Reset player state to defaults when quitting mid-game
                if let Some(ref mut state) = player_state {
                    state.reset_to_defaults();
                    info!("Quit button pressed - resetting game state to defaults");
                }
                
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

pub fn handle_pull_orb_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<PullOrbButton>),
    >,
    mut player_state: Option<ResMut<crate::game_state::PlayerGameState>>,
) {
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(Color::srgb(0.1, 0.2, 0.1));
                *border_color = BorderColor(Color::srgb(0.2, 0.4, 0.2));
                
                if let Some(ref mut state) = player_state {
                    state.pull_orb();
                }
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::srgb(0.3, 0.5, 0.3));
                *border_color = BorderColor(Color::srgb(0.5, 0.7, 0.5));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::srgb(0.2, 0.4, 0.2));
                *border_color = BorderColor(Color::srgb(0.4, 0.6, 0.4));
            }
        }
    }
}

pub fn cleanup_playing(mut commands: Commands, playing_query: Query<Entity, With<PlayingUI>>) {
    for entity in &playing_query {
        commands.entity(entity).despawn();
    }
}

pub fn update_pull_history(
    mut commands: Commands,
    player_state: Option<Res<crate::game_state::PlayerGameState>>,
    history_container: Query<Entity, With<PullHistoryContainer>>,
    existing_orbs: Query<Entity, With<PullHistoryOrb>>,
) {
    let Some(state) = player_state else { return };
    
    // Clear existing orb displays
    for entity in &existing_orbs {
        commands.entity(entity).despawn();
    }
    
    // Get the history container
    let Ok(container) = history_container.single() else { return };
    
    // Create new orb displays
    commands.entity(container).with_children(|parent| {
        for (index, orb) in state.pull_history().iter().enumerate() {
            let (color, symbol) = match orb {
                Orb::Health => (Color::srgb(0.2, 0.8, 0.2), "H"),
                Orb::Point => (Color::srgb(0.2, 0.2, 0.8), "P"),
                Orb::Bomb => (Color::srgb(0.8, 0.2, 0.2), "B"),
            };
            
            parent.spawn((
                Node {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(color.with_alpha(0.3)),
                BorderColor(color),
                PullHistoryOrb { _index: index },
            ))
            .with_children(|orb_parent| {
                orb_parent.spawn((
                    Text::new(symbol),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        }
        
        // Add empty slots for remaining history
        for index in state.pull_history().len()..5 {
            parent.spawn((
                Node {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.2)),
                BorderColor(Color::srgba(0.4, 0.4, 0.4, 0.3)),
                PullHistoryOrb { _index: index },
            ))
            .with_children(|orb_parent| {
                orb_parent.spawn((
                    Text::new("?"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgba(0.5, 0.5, 0.5, 0.5)),
                ));
            });
        }
    });
}