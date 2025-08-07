use bevy::prelude::*;
use super::{GameState, PlayingUI, QuitButton, PullOrbButton, StatDisplay, StatType, PullHistoryContainer, PullHistoryOrb};
use crate::game_state::orb::Orb;

pub fn setup_playing_ui(
    mut commands: Commands,
    player_state: Option<Res<crate::game_state::PlayerGameState>>,
) {
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
                    width: Val::Px(190.0), // Container width for 5 orbs (30px each + 10px gaps)
                    height: Val::Px(30.0),
                    position_type: PositionType::Relative,
                    overflow: Overflow::clip(), // Hide orbs that slide out
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.2)),
                PullHistoryContainer { last_total_pulls: player_state.as_ref().map(|s| s.total_pulls()).unwrap_or(0) },
            ))
            .with_children(|container_parent| {
                // Initialize with existing pull history if any
                if let Some(ref state) = player_state {
                    for (index, orb) in state.pull_history().iter().enumerate() {
                        let (color, symbol) = match orb {
                            Orb::Health => (Color::srgb(0.2, 0.8, 0.2), "H"),
                            Orb::Point => (Color::srgb(0.2, 0.2, 0.8), "P"),
                            Orb::Bomb => (Color::srgb(0.8, 0.2, 0.2), "B"),
                        };
                        
                        container_parent.spawn((
                            Node {
                                width: Val::Px(30.0),
                                height: Val::Px(30.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                position_type: PositionType::Absolute,
                                left: Val::Px(index as f32 * 40.0), // Position based on index
                                ..default()
                            },
                            BackgroundColor(color.with_alpha(0.3)),
                            BorderColor(color),
                            PullHistoryOrb {
                                _orb_type: *orb,
                                position: index,
                                animation_timer: 1.0, // Already in position
                            },
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
                }
            });
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
    mut history_container: Query<(Entity, &mut PullHistoryContainer)>,
    existing_orbs: Query<Entity, With<PullHistoryOrb>>,
    _time: Res<Time>,
) {
    let Some(state) = player_state else { return };
    let Ok((container_entity, mut container)) = history_container.single_mut() else { return };
    
    let current_total_pulls = state.total_pulls();
    let last_tracked_pulls = container.last_total_pulls;
    
    // Check if a new pull was made
    if current_total_pulls > last_tracked_pulls {
        // Clear all existing orbs and rebuild from history
        for entity in &existing_orbs {
            commands.entity(entity).despawn();
        }
        
        // Update the container's tracking
        container.last_total_pulls = current_total_pulls;
        
        // Rebuild all orbs from current history with animations
        let history = state.pull_history();
        commands.entity(container_entity).with_children(|parent| {
            for (index, orb) in history.iter().enumerate() {
                let (color, symbol) = match orb {
                    Orb::Health => (Color::srgb(0.2, 0.8, 0.2), "H"),
                    Orb::Point => (Color::srgb(0.2, 0.2, 0.8), "P"),
                    Orb::Bomb => (Color::srgb(0.8, 0.2, 0.2), "B"),
                };
                
                // Check if this is the new orb (rightmost)
                let is_new_orb = index == history.len() - 1;
                let start_x = if is_new_orb { 200.0 } else { index as f32 * 40.0 };
                
                parent.spawn((
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        position_type: PositionType::Absolute,
                        left: Val::Px(start_x),
                        ..default()
                    },
                    BackgroundColor(color.with_alpha(0.3)),
                    BorderColor(color),
                    PullHistoryOrb {
                        _orb_type: *orb,
                        position: index,
                        animation_timer: if is_new_orb { 0.0 } else { 1.0 },
                    },
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
        });
    }
    
    // Clear all orbs if history is empty (level reset)
    if current_total_pulls == 0 && !existing_orbs.is_empty() {
        for entity in &existing_orbs {
            commands.entity(entity).despawn();
        }
        container.last_total_pulls = 0;
    }
}

pub fn animate_pull_history(
    mut orbs: Query<(&mut Node, &mut PullHistoryOrb)>,
    time: Res<Time>,
) {
    for (mut node, mut orb_data) in &mut orbs {
        // Animate the sliding effect
        orb_data.animation_timer += time.delta_secs();
        
        // Position orbs from left to right based on their position index
        let target_x = orb_data.position as f32 * 40.0; // 30px width + 10px gap
        let current_x = node.left.resolve(0.0, Vec2::ZERO).unwrap_or(200.0);
        
        // Smooth interpolation
        let t = (orb_data.animation_timer * 5.0).min(1.0); // Animation takes 0.2 seconds
        let new_x = current_x + (target_x - current_x) * t;
        
        node.left = Val::Px(new_x);
    }
}